use super::*;

#[derive(new, Default, Debug)]
pub struct ActionAction {
    name: String,
    actions: Vec<String>,
}

impl ActionImpl for ActionAction {
    fn execute(&self, logger: String) -> Result<()> {
        for action in &self.actions {
            task::block_on(CERBERUS.execute(logger.clone(), action.clone()))?;
        }
        Ok(())
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
