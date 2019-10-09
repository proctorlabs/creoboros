use super::*;

#[derive(Debug, Clone, new)]
pub struct Start {
    name: String,
    logger: String,
    actions: Vec<String>,
}

impl ModuleExt for Start {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn initialize(&mut self, _: &Sender<Message>) -> Result<()> {
        task::block_on(async {
            for action in self.actions.iter() {
                crate::RT
                    .execute(self.logger.clone(), action.to_string())
                    .await?;
            }
            Ok(())
        })
    }

    fn priority(&self) -> u16 {
        10
    }
}
