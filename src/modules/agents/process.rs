use super::*;
use crate::config::Policy;
use crate::templates::ConfigTemplate;
use async_std::fs::File;
use async_std::io::BufReader;
use async_std::os::unix::io::*;
use std::os::unix::io::IntoRawFd;
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Debug, Clone, new)]
pub struct Process {
    name: String,
    command: ConfigTemplate,
    args: Vec<ConfigTemplate>,
    logger: String,
    policy: Policy,
}

impl ModuleExt for Process {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, _: Message) -> Result<()> {
        Ok(())
    }

    fn initialize(&mut self, sender: &Sender<Message>) -> Result<()> {
        let args = self
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

        let md = (self.name.clone(), self.logger.clone());
        monitor_fd_lines(
            child.stdout.take(),
            move |s| info!("{}" [s.trim()] agent: md.0 => md.1),
        )?;

        let md = (self.name.clone(), self.logger.clone());
        monitor_fd_lines(
            child.stderr.take(),
            move |s| warn!("{}" [s.trim()] agent: md.0 => md.1),
        )?;

        let mut md = (
            self.name.clone(),
            self.logger.clone(),
            sender.clone(),
            self.policy.clone(),
            self.clone(),
        );

        task::spawn(async move {
            let mut res;
            loop {
                res = child.try_wait();
                match res {
                    Ok(None) => task::sleep(Duration::from_millis(500)).await,
                    _ => break,
                }
            }
            let status = res?.unwrap();
            // let status = async_process::ChildWait(child).await?;
            warn!("Process exited with status: {}" [status] agent: md.0, reason: status => md.1);
            match md.3 {
                Policy::Restart { delay } => {
                    info!("Attempting to restart process in {} seconds..."[delay] agent: md.0 => md.1);
                    task::sleep(Duration::from_secs(delay)).await;
                    md.4.initialize(&md.2)?;
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

fn monitor_fd_lines<T: IntoRawFd>(
    mut fd_opt: Option<T>,
    cb: impl Fn(String) + 'static + Send + Sync,
) -> Result<()> {
    let fd = fd_opt
        .take()
        .ok_or(AppError::Critical {
            message: "Failed to capture output!".into(),
        })?
        .into_raw_fd();
    task::spawn(async move {
        let f = unsafe { File::from_raw_fd(fd) };
        let mut buf = BufReader::new(f);
        loop {
            let mut line = String::new();
            let res = buf.read_line(&mut line).await;
            if let Ok(c) = res {
                if c > 0 {
                    (cb)(line);
                    continue;
                }
            }
            break;
        }
    });
    Ok(())
}

// This below is an experimental async wrapper for the wak above to eliminate the sleep/try_wait loop above

// mod async_process {
//     use std::future::Future;
//     use std::io;
//     use std::pin::Pin;
//     use std::process::ExitStatus;
//     use std::task::{Context, Poll};

//     pub struct ChildWait(pub std::process::Child);

//     impl Future for ChildWait {
//         type Output = io::Result<ExitStatus>;

//         fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//             match Pin::new(&mut self.0).try_wait() {
//                 Ok(None) => {
//                     Poll::Pending
//                 }
//                 Ok(Some(val)) => Poll::Ready(Ok(val)),
//                 Err(e) => Poll::Ready(Err(e)),
//             }
//         }
//     }
// }
