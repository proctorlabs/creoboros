use super::*;
use crate::config::Policy;
use crate::templates::ConfigTemplate;
use async_std::fs::File;
use async_std::io::BufReader;
use async_std::os::unix::io::*;
use std::os::unix::io::IntoRawFd;
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Process {
    name: String,
    command: ConfigTemplate,
    args: Vec<ConfigTemplate>,
    logger: String,
    policy: Policy,
}

impl Process {
    pub fn new(
        name: String,
        command: ConfigTemplate,
        args: Vec<ConfigTemplate>,
        logger: String,
        policy: Policy,
    ) -> Self {
        Process {
            name,
            command,
            args,
            logger,
            policy,
        }
    }
}

impl ModuleExt for Process {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, _: Message) -> Result<()> {
        Ok(())
    }

    fn initialize(&self, sender: &Sender<Message>) -> Result<()> {
        let args: Vec<String> = self
            .args
            .iter()
            .map(|e| Ok(e.render()?))
            .collect::<Result<Vec<String>>>()?;
        let mut child = Command::new(self.command.render()?)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child
            .stdout
            .take()
            .ok_or(AppError::Critical {
                message: format!(
                    "Failed to start agent {}, failed to capture stdout",
                    self.name
                ),
            })?
            .into_raw_fd();

        let stderr = child
            .stderr
            .take()
            .ok_or(AppError::Critical {
                message: format!(
                    "Failed to start agent {}, failed to capture stderr",
                    self.name
                ),
            })?
            .into_raw_fd();

        let md = (self.name.clone(), self.logger.clone());
        task::spawn(async move {
            let f = unsafe { File::from_raw_fd(stdout) };
            let mut buf = BufReader::new(f);
            loop {
                let mut line = String::new();
                let res = buf.read_line(&mut line).await;
                if let Ok(c) = res {
                    if c > 0 {
                        info!("{}" [line.trim()] agent: md.0 => md.1);
                        continue;
                    }
                }
                break;
            }
        });

        let md = (self.name.clone(), self.logger.clone());
        task::spawn(async move {
            let f = unsafe { File::from_raw_fd(stderr) };
            let mut buf = BufReader::new(f);
            loop {
                let mut line = String::new();
                let res = buf.read_line(&mut line).await;
                if let Ok(c) = res {
                    if c > 0 {
                        warn!("{}" [line.trim()] agent: md.0 => md.1);
                        continue;
                    }
                }
                break;
            }
        });

        let md = (
            self.name.clone(),
            self.logger.clone(),
            sender.clone(),
            self.policy.clone(),
            self.clone(),
        );
        task::spawn(async move {
            let status = child.wait()?;
            warn!("Process exited with status: {}" [status] agent: md.0, reason: status => md.1);
            match md.3 {
                Policy::Restart { delay } => {
                    info!("Attempting to restart process in {} seconds..."[delay] agent: md.0 => md.1);
                    task::spawn(async move {
                        task::sleep(Duration::from_secs(delay)).await;
                        md.4.initialize(&md.2)
                    });
                }
                Policy::Nothing => {
                    info!("Process exited, 'nothing' policy implies no action taken");
                }
            }
            Ok::<(), AppError>(())
        });
        Ok(())
    }
}
