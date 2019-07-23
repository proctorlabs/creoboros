use super::*;
use std::io::BufReader;
use std::process::{Command, Stdio};
use tokio_process::CommandExt;

impl RunnableAgent for Arc<super::Service> {
    fn execute(&self) -> Result<()> {
        let mut child = Command::new(self.command.clone())
            .args(self.args.clone())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn_async()?;

        let stdout = child.stdout().take().ok_or(AppError::Critical {
            message: format!(
                "Failed to start agent {}, failed to capture stdout",
                self.name
            ),
        })?;

        let stderr = child.stderr().take().ok_or(AppError::Critical {
            message: format!(
                "Failed to start agent {}, failed to capture stderr",
                self.name
            ),
        })?;

        capture!(self:slf {
            spawn!(
                tokio::io::lines(BufReader::new(stdout)).for_each(move |line| {
                    info!("{}" [&line] agent: slf.name => slf.logger);
                    Ok(())
                })
            )?;
        });

        capture!(self:slf {
            spawn!(
                tokio::io::lines(BufReader::new(stderr)).for_each(move |line| {
                    warn!("{}" [&line] agent: slf.name => slf.logger);
                    Ok(())
                })
            )?;
        });

        capture!(self:slf1, self:slf2 {
            spawn!(child
                .map(move |status| info!("Process exited with status: {}" [status] agent: slf1.name, reason: status => slf1.logger))
                .map_err(move |e| warn!("Failed to start process: {}" [e] agent: slf2.name => slf2.logger)))
        })
    }
}
