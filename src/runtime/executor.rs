use super::*;
use std::io::{self, Write};
use std::process::Command;
use tokio_process::CommandExt;

impl Boomslang {
    pub fn execute(&self, task: crate::agents::Executor) -> Result<()> {
        self.spawn(lazy(move || {
            Command::new("/bin/sh")
                .arg("-c")
                .arg(&task.path)
                .output_async()
                .into_future()
                .map(|output| {
                    info!("Exited with status {}", output.status);
                    io::stdout().write_all(&output.stdout).unwrap();
                    io::stderr().write_all(&output.stderr).unwrap();
                })
                .map_err(|e| warn!("Error occurred! {:?}", e))
        }))
    }
}
