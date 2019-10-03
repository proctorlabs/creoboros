use super::*;

use std::time::Duration;

pub struct Timer {
    name: String,
    interval: Duration,
    logger: String,
}

impl Timer {
    pub fn new(name: String, interval: Duration, logger: String) -> Self {
        Timer {
            name,
            interval,
            logger,
        }
    }
}

impl ModuleExt for Timer {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn initialize(&self, sender: &Sender<Message>) -> Result<()> {
        let interval = self.interval;
        let s = sender.clone();
        task::spawn(async move {
            while let Ok(_) = s.send(Message::Init) {
                task::sleep(interval).await;
            }
        });
        Ok(())
    }

    fn handle(&self, _: Message) -> Result<()> {
        info!("Timer event occurred" agent: self.name => self.logger);
        Ok(())
    }
}
