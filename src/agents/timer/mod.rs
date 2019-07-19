use super::*;

use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::Instant;

impl RunnableAgent for Arc<Timer> {
    fn execute(&self) -> Result<()> {
        let zelf = self.clone();
        spawn!(
            Interval::new(Instant::now() + self.interval, self.interval).for_each(move |_| {
                info!(target: &zelf.logger, "Timer event occurred");
                Ok(())
            })
        )
    }
}
