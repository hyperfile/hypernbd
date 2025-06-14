use nbdkit::*;

struct HyperNbd {
}

impl HyperNbd {
    fn new() -> Self {
        Self {}
    }
}

impl Server for HyperNbd {
    fn get_size(&self) -> Result<i64> {
        todo!();
    }

    fn name() -> &'static str {
        "hypernbd"
    }

    fn open(readonly: bool) -> Result<Box<dyn Server>> {
        Ok(Box::new(HyperNbd::new()))
    }

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<()> {
        todo!();
    }
}

plugin!(HyperNbd {});
