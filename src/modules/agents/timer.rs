use super::*;

use std::time::Duration;

#[derive(Debug, new)]
pub struct Timer {
    name: String,
    interval: Duration,
    logger: String,
    actions: Vec<String>,
}

impl ModuleExt for Timer {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn initialize(&self, sender: &Sender<Message>) -> Result<()> {
        let interval = self.interval;
        let s = sender.clone();
        task::spawn(async move {
            while let Ok(_) = s.send(Message::Unit) {
                task::sleep(interval).await;
            }
        });
        Ok(())
    }

    fn handle(&self, _: Message) -> Result<()> {
        for action in self.actions.iter() {
            crate::CERBERUS.execute(self.logger.clone(), action.to_string())?;
        }
        Ok(())
    }
}
