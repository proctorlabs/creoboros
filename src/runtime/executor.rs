use super::*;
use std::io::BufReader;
use std::process::{Command, Stdio};
use tokio_process::CommandExt;

impl Boomslang {
    pub fn execute(&self, task: crate::agents::Executor) -> Result<()> {
        let mut child = Command::new(task.command)
            .args(task.args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn_async()?;

        let stdout = child.stdout().take().ok_or(AppError::Critical {
            message: "Failed to start child process!".into(),
        })?;

        let stderr = child.stderr().take().ok_or(AppError::Critical {
            message: "Failed to start child process!".into(),
        })?;

        let child_future = child
            .map(|status| info!("Process exited with status: {}", status))
            .map_err(|e| warn!("Failed to start process: {}", e));

        let stdout_reader = tokio_io::io::lines(BufReader::new(stdout))
            .for_each(|line| {
                info!("Output: {}", line);
                Ok(())
            })
            .map_err(|e| warn!("{:?}", e));

        let stderr_reader = tokio_io::io::lines(BufReader::new(stderr))
            .for_each(|line| {
                warn!("Error: {}", line);
                Ok(())
            })
            .map_err(|e| warn!("{:?}", e));

        self.spawn(stdout_reader)?;
        self.spawn(stderr_reader)?;
        self.spawn(child_future)
    }
}
