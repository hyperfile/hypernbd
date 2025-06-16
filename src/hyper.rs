use std::io::Result;
use std::io::SeekFrom;
use std::sync::Arc;
use log::debug;
use tokio::sync::RwLock;
use tokio::io::{AsyncSeekExt, AsyncReadExt, AsyncWriteExt};
use tokio::runtime::Runtime;
use aws_sdk_s3::Client;
use hyperfile::file::tokio_wrapper::HyperFileTokio;
use hyperfile::file::flags::FileFlags;
use hyperfile::file::mode::FileMode;

pub(crate) struct HyperNbd<'a> {
    uri: String,
    rt: Runtime,
    client: Client,
    file: Arc<RwLock<HyperFileTokio<'a>>>,
}

impl<'a: 'static> HyperNbd<'a> {
    pub(crate) fn open(uri: &str, readonly: bool) -> Result<Self> {
        debug!("open back device: {}", uri);
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let (file, client) = rt.block_on(async move {
            let config = aws_config::load_from_env().await;
            let client = aws_sdk_s3::Client::new(&config);

            let flags = if readonly {
                FileFlags::rdonly()
            } else {
                FileFlags::rdwr()
            };
            let mode = FileMode::default_file();
            (HyperFileTokio::open_or_create_with_default_opt(&client, uri, flags, mode).await, client)
        });

        Ok(Self {
            uri: uri.to_owned(),
            rt,
            client,
            file: Arc::new(RwLock::new(file?)),
        })
    }

    pub(crate) fn get_volume_size(&self) -> Result<i64> {
        let file = self.file.clone();
        let res = self.rt.handle().block_on(async {
            file.read().await.metadata().await
        });
        res.map(|stat| stat.st_size as i64)
    }

    pub(crate) fn read(&self, offset: u64, buf: &mut [u8]) -> Result<()> {
        let file = self.file.clone();
        let res = self.rt.handle().block_on(async {
            let mut lock = file.write().await;
            let _ = lock.seek(SeekFrom::Start(offset)).await?;
            lock.read_exact(buf).await
        });
        res.map(|_read_size| ())
    }

    pub(crate) fn write(&self, offset: u64, buf: &[u8]) -> Result<()> {
        let file = self.file.clone();
        let res = self.rt.handle().block_on(async {
            let mut lock = file.write().await;
            let _ = lock.seek(SeekFrom::Start(offset)).await?;
            lock.write_all(buf).await
        });
        res.map(|_write_size| ())
    }

    pub(crate) fn write_zero(&self, offset: u64, count: u32) -> Result<()> {
        let file = self.file.clone();
        let res = self.rt.handle().block_on(async {
            let mut lock = file.write().await;
            let _ = lock.seek(SeekFrom::Start(offset)).await?;
            lock.write_zero(count as usize).await
        });
        res.map(|_write_size| ())
    }
}
