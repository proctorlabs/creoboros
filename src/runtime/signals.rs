use super::Creoboros;
use crate::prelude::*;
use signal_hook::{iterator::Signals, SIGINT, SIGQUIT, SIGTERM};

impl Creoboros {
    pub fn add_signal_hooks(&self) -> Result<()> {
        let signals = Signals::new(&[SIGINT, SIGTERM, SIGQUIT])?;
        for s in signals.wait() {
            if s == SIGTERM || s == SIGINT || s == SIGQUIT {
                break;
            }
        }
        Ok(())
    }
}
