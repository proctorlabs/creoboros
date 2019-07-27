use super::Cerberus;
use crate::prelude::*;
use signal_hook::{iterator::Signals, SIGINT, SIGQUIT, SIGTERM};

impl Cerberus {
    pub fn add_signal_hooks(&self) -> Result<()> {
        self.wait(
            Signals::new(&[SIGINT, SIGTERM, SIGQUIT])?
                .into_async()?
                .into_future()
                .map(|_| info!("Signal received! Shutting down..." agent: "master"))
                .map_err(|e| warn!("{}"[e.0] agent: "master")),
        )
    }
}
