use super::*;
use std::io::BufReader;
use std::process::{Command, Stdio};
use tokio_process::CommandExt;

impl RunnableAgent for super::Executor {
    fn execute(self) -> Result<()> {
        let mut send_name;
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

        send_name = self.name.to_owned();
        spawn!(
            tokio::io::lines(BufReader::new(stdout)).for_each(move |line| {
                log_event!(send_name.clone(), "stdout" => line);
                Ok(())
            })
        )?;

        send_name = self.name.to_string();
        spawn!(
            tokio::io::lines(BufReader::new(stderr)).for_each(move |line| {
                log_event!(send_name.clone(), "stderr" => line);
                Ok(())
            })
        )?;

        send_name = self.name.to_string();
        let send_name2 = send_name.to_string();
        spawn!(child
            .map(move |status| log_event!(send_name.clone(), "process" => format!("Process exited with status: {}", status)))
            .map_err(move |e| log_event!(send_name2.clone(), "process" => format!("Failed to start process: {}", e)))
        )
    }
}
