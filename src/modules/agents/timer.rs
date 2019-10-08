use super::*;

use std::time::Duration;

#[derive(Debug, Clone, new)]
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

    fn initialize(&mut self, sender: &Sender<Message>) -> Result<()> {
        let interval = self.interval;
        let s = sender.clone();
        task::spawn(async move {
            while let Ok(_) = s.unbounded_send(Message::Unit) {
                task::sleep(interval).await;
            }
        });
        Ok(())
    }

    fn handle(&self, _: Message) -> Result<()> {
        task::block_on(async {
            for action in self.actions.iter() {
                crate::CERBERUS
                    .execute(self.logger.clone(), action.to_string())
                    .await?;
            }
            Ok(())
        })
    }

    fn priority(&self) -> u16 {
        200
    }
}
