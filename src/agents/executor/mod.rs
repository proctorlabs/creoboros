use super::*;
use std::io::BufReader;
use std::process::{Command, Stdio};
use tokio_process::CommandExt;

impl RunnableAgent for super::Executor {
    fn execute(self) -> Result<()> {
        let mut sender;
        let mut child = Command::new(self.command)
            .args(self.args)
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

        sender = BOOMSLANG.sender();
        spawn!(
            tokio::io::lines(BufReader::new(stdout)).for_each(move |line| {
                sender
                    .try_send(Log {
                        log: line.to_string(),
                    })
                    .unwrap_or_default();
                Ok(())
            })
        )?;

        sender = BOOMSLANG.sender();
        spawn!(
            tokio::io::lines(BufReader::new(stderr)).for_each(move |line| {
                sender
                    .try_send(Log {
                        log: line.to_string(),
                    })
                    .unwrap_or_default();
                Ok(())
            })
        )?;

        spawn!(child
            .map(|status| info!("Process exited with status: {}", status))
            .map_err(|e| warn!("Failed to start process: {}", e)))
    }
}
