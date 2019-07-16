use super::*;

use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::Instant;

impl RunnableAgent for Timer {
    fn execute(self) -> Result<()> {
        spawn!(
            Interval::new(Instant::now() + self.interval, self.interval).for_each(|_| {
                info!("Timer called!");
                Ok(())
            })
        )
    }
}
