use std::io::{Result, Error, ErrorKind};
use std::sync::Arc;
use std::sync::OnceLock;
use log::debug;
use tokio::sync::oneshot;
use tokio::runtime::Runtime;
use aws_sdk_s3::Client;
use reactor::{TaskHandler, LocalSpawner};
use hyperfile::file::hyper::Hyper;
use hyperfile::file::handler::FileContext;
use hyperfile::file::flags::FileFlags;
use hyperfile::file::mode::FileMode;
use hyperfile::config::HyperFileConfigBuilder;
use hyperfile::staging::config::StagingConfig;
use hyperfile::wal::config::HyperFileWalConfig;
use hyperfile::config::{HyperFileMetaConfig, HyperFileRuntimeConfig};

pub(crate) static BACKEND_RUNTIME: OnceLock<Arc<Runtime>> = OnceLock::new();
pub(crate) static BACKEND_HYPER: OnceLock<TaskHandler<FileContext<'_>>> = OnceLock::new();

pub(crate) struct HyperNbd<'a> {
    uri: String,
    rt: Arc<Runtime>,
    client: Client,
    spawner: LocalSpawner<FileContext<'a>, Hyper<'a>>,
    handler: TaskHandler<FileContext<'a>>,
}

impl<'a: 'static> HyperNbd<'a> {
    pub(crate) fn open(uri: &str, wal_uri: &str, readonly: bool) -> Result<Self> {
        debug!("open back device: {}", uri);
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let res = rt.block_on(async move {
            let config = aws_config::load_from_env().await;
            let client = aws_sdk_s3::Client::new(&config);

            let flags = if readonly {
                FileFlags::rdonly()
            } else {
                FileFlags::rdwr()
            };
            let mode = FileMode::default_file();

            let meta_config = HyperFileMetaConfig::default();
            let staging_config = StagingConfig::new_s3_uri(uri, None);
            let runtime_config = HyperFileRuntimeConfig::default_large();
            let wal_config = HyperFileWalConfig::new(wal_uri);
            let file_config = HyperFileConfigBuilder::new()
                                .with_meta_config(&meta_config)
                                .with_staging_config(&staging_config)
                                .with_runtime_config(&runtime_config)
                                .with_wal_config(&wal_config)
                                .build();
            let spawner = LocalSpawner::new_current();
            let (tx, rx) = oneshot::channel();
            match Hyper::fs_open_or_create_with_config(&client, file_config, flags, mode).await {
                Ok(hyper) => {
                    spawner.spawn(hyper, tx);
                    let fh = rx.await.expect("failed to get back file handler");
                    return Ok((spawner, fh, client));
                },
                Err(e) => {
                    return Err(e);
                },
            }
        });

        let (spawner, handler, client) = res?;

        // save runtime handle into global var
        let rt = Arc::new(rt);
        let res = BACKEND_RUNTIME.set(rt.clone());
        if res.is_err() {
            return Err(Error::new(ErrorKind::ResourceBusy, "failed to set backend runtime"));
        }

        // save handler into global var
        let res = BACKEND_HYPER.set(handler.clone());
        if res.is_err() {
            return Err(Error::new(ErrorKind::ResourceBusy, "failed to set backend hyper"));
        }

        Ok(Self {
            uri: uri.to_owned(),
            rt,
            client,
            spawner,
            handler,
        })
    }

    pub(crate) fn shutdown_runtime() {
        // shutdown runtime
        if let Some(arc_rt) = BACKEND_RUNTIME.get() {
            // ugly clean up runtime reference by unsafe code
            let arc_rt_clone = arc_rt.clone();
            let mut strong_count = Arc::strong_count(&arc_rt_clone);
            let raw = Arc::into_raw(arc_rt_clone.clone());
            while strong_count > 0 {
                unsafe {
                    Arc::decrement_strong_count(raw);
                }
                strong_count -= 1;
            }
            let arc_rt = unsafe { Arc::from_raw(raw) };
            if let Some(rt) = Arc::into_inner(arc_rt) {
                rt.shutdown_background();
                debug!("runtime shutdown completed");
            }
        }
    }

    pub(crate) fn shutdown() {
        // get back runtime and handler
        let Some(rt) = BACKEND_RUNTIME.get() else {
            debug!("backend runtime is uninitialized");
            return;
        };
        let Some(handler) = BACKEND_HYPER.get() else {
            debug!("backend hyper is uninitialized");
            Self::shutdown_runtime();
            return;
        };
        let _ = rt.handle().block_on(async {
            // do release
            let (ctx, rx) = FileContext::new_release();
            handler.send(ctx);
            rx.await.expect("task channel closed")
        });
        debug!("handler shutdown completed");
        Self::shutdown_runtime();
    }

    pub(crate) fn get_volume_size(&self) -> Result<i64> {
        let (ctx, rx) = FileContext::new_getattr();
        self.handler.send(ctx);
        let res = self.rt.handle().block_on(async {
            rx.await.expect("task channel closed")
        });
        res.map(|stat| stat.st_size as i64)
    }

    pub(crate) fn read(&self, offset: u64, buf: &mut [u8]) -> Result<()> {
        let b = unsafe { std::slice::from_raw_parts_mut(buf.as_ptr() as *mut u8, buf.len()) };
        let (ctx, tx, mut rx) = FileContext::new_read(b, offset as usize, self.handler.clone());
        self.handler.send(ctx);
        let res = self.rt.handle().block_on(async {
            rx.recv().await.expect("task channel closed")
        });
        drop(tx);
        res.map(|_read_size| ())
    }

    pub(crate) fn write(&self, offset: u64, buf: &[u8]) -> Result<()> {
        let b = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, buf.len()) };
        let (ctx, tx, mut rx) = FileContext::new_write(b, offset as usize, self.handler.clone());
        self.handler.send(ctx);
        let res = self.rt.handle().block_on(async {
            rx.recv().await.expect("task channel closed")
        });
        drop(tx);
        res.map(|_write_size| ())
    }

    pub(crate) fn write_zero(&self, offset: u64, count: u32) -> Result<()> {
        let (ctx, tx, mut rx) = FileContext::new_write_zero(offset as usize, count as usize, self.handler.clone());
        self.handler.send(ctx);
        let res = self.rt.handle().block_on(async {
            rx.recv().await.expect("task channel closed")
        });
        drop(tx);
        res.map(|_write_size| ())
    }

    pub(crate) fn do_flush(&self) -> Result<()> {
        let (ctx, rx) = FileContext::new_flush();
        self.handler.send(ctx);
        let res = self.rt.handle().block_on(async {
            rx.await.expect("task channel closed")
        });
        res.map(|_cno| ())
    }
}
