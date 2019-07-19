use super::Boomslang;
use crate::prelude::*;
use signal_hook::{iterator::Signals, SIGINT, SIGQUIT, SIGTERM};

impl Boomslang {
    pub fn add_signal_hooks(&self) -> Result<()> {
        self.wait(
            Signals::new(&[SIGINT, SIGTERM, SIGQUIT])?
                .into_async()?
                .into_future()
                .map(|sig| info!("{:?}"[sig.0]))
                .map_err(|e| warn!("{}"[e.0])),
        )
    }
}
