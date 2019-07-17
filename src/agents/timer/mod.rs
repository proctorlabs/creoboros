use super::*;

use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::Instant;

impl RunnableAgent for Timer {
    fn execute(self) -> Result<()> {
        let send_name = self.name.to_string();
        spawn!(
            Interval::new(Instant::now() + self.interval, self.interval).for_each(move |_| {
                log_event!(send_name.clone(), "timer" => "Timer event occurred");
                Ok(())
            })
        )
    }
}
