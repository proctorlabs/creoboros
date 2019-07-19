use super::*;

use std::time::Instant;
use tokio::prelude::*;
use tokio::timer::Interval;

impl RunnableAgent for Arc<Timer> {
    fn execute(&self) -> Result<()> {
        let zelf = self.clone();
        spawn!(
            Interval::new(Instant::now() + self.interval, self.interval).for_each(move |_| {
                info!("Timer event occurred" agent: zelf.name => zelf.logger);
                Ok(())
            })
        )
    }
}
