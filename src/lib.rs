use std::sync::OnceLock;
use nbdkit::*;
use log::debug;

mod hyper;

use hyper::HyperNbd;

static BACKEND_URI: OnceLock<String> = OnceLock::new();

impl<'a: 'static> Server for HyperNbd<'a> {
    // required
    fn get_size(&self) -> Result<i64> {
        debug!("get_size -");
        Ok(self.get_volume_size()?)
    }

    fn name() -> &'static str {
        "hypernbd"
    }

    fn open(readonly: bool) -> Result<Box<dyn Server>> {
        debug!("open - readonly: {}", readonly);
        let Some(uri) = BACKEND_URI.get() else {
            return Err(Error::new(libc::EINVAL, "failed to get backend uri"));
        };
        Ok(Box::new(HyperNbd::open(uri, readonly)?))
    }

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<()> {
        debug!("read_at - offset: {}, len: {}", offset, buf.len());
        Ok(self.read(offset, buf)?)
    }

    // overwrite provided
    fn is_rotational(&self) -> Result<bool> {
        debug!("is_rotational - set false");
        Ok(false)
    }

    fn load() {
        env_logger::init();
        debug!("load -");
    }

    fn unload() {
        debug!("unload -");
    }

    fn dump_plugin() {
        debug!("dump plugin -");
    }

    fn config(key: &str, value: &str) -> Result<()> {
        debug!("config - key: {}, value: {}", key, value);
        if key == "backend_uri" {
            // TODO: validate uri
            let res = BACKEND_URI.set(value.to_string());
            if res.is_err() {
                return Err(Error::new(libc::EINVAL, "failed to set backend uri from command line input"));
            }
        }
        Ok(())
    }

    fn config_complete() -> Result<()> {
        debug!("config_complete -");
        Ok(())
    }

    fn write_at(&self, buf: &[u8], offset: u64, flags: Flags) -> Result<()> {
        debug!("write_at - offset: {}, len: {}, flags: {}", offset, buf.len(), flags.bits());
        Ok(self.write(offset, buf)?)
    }
}

plugin!(HyperNbd {is_rotational, load, unload, dump_plugin, config, config_complete, write_at});
