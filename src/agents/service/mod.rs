use super::*;
use async_std::fs::File;
use async_std::io::BufReader;
use async_std::os::unix::io::*;
use std::os::unix::io::IntoRawFd;
use std::process::{Command, Stdio};

impl RunnableAgent for Arc<super::Service> {
    fn execute(&self) -> Result<()> {
        let mut child = Command::new(self.command.clone())
            .args(self.args.clone())
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

        capture!(self:slf {
            task::spawn(async move {
                let f = unsafe { File::from_raw_fd(stdout) };
                let mut buf = BufReader::new(f);
                loop {
                    let mut line = String::new();
                    let res = buf.read_line(&mut line).await;
                    if let Ok(c) = res {
                        if c > 0 {
                            info!("{}" [&line] agent: slf.name => slf.logger);
                            continue
                        }
                    }
                    break
                }
            });
        });

        capture!(self:slf {
            task::spawn(async move {
                let f = unsafe { File::from_raw_fd(stderr) };
                let mut buf = BufReader::new(f);
                loop {
                    let mut line = String::new();
                    let res = buf.read_line(&mut line).await;
                    if let Ok(c) = res {
                        if c > 0 {
                            warn!("{}" [&line] agent: slf.name => slf.logger);
                            continue
                        }
                    }
                    break
                }
            });
        });

        capture!(self:slf1, self:slf3 {
            task::spawn(async move {
                let status = child.wait()?;
                warn!("Process exited with status: {}" [status] agent: slf1.name, reason: status => slf1.logger);
                match slf1.policy {
                    Policy::Restart{delay} => {
                        info!("Attempting to restart process in {} seconds..."[delay] agent: slf1.name => slf1.logger);
                        task::spawn(async move {
                            task::sleep(Duration::from_secs(delay)).await;
                            slf3.execute()
                        });
                    }
                    Policy::Nothing => {
                        info!("Process exited, 'nothing' policy implies no action taken");
                    }
                }
                Ok::<(), AppError>(())
            });
        });
        Ok(())
    }
}
