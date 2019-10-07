use super::*;

use std::process::Command;

#[derive(new, Default, Debug)]
pub struct Run {
    name: String,
    command: ConfigTemplate,
    args: Vec<ConfigTemplate>,
}

impl ActionImpl for Run {
    fn execute(&self, logger: String) -> Result<()> {
        let args: Vec<String> = self
            .args
            .iter()
            .map(|e| Ok(e.render()?))
            .collect::<Result<Vec<String>>>()?;
        let child = Command::new(self.command.render()?).args(args).output()?;
        if child.status.code() != Some(0) {
            warn!("Script exited with status {}!"[child.status] => logger);
        }
        if !child.stdout.is_empty() {
            info!("{}"[String::from_utf8(child.stdout).unwrap_or_default()] => logger)
        }
        if !child.stderr.is_empty() {
            info!("{}"[String::from_utf8(child.stderr).unwrap_or_default()] => logger)
        }
        Ok(())
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
