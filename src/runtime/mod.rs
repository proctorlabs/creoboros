mod executor;
mod signals;

use crate::prelude::*;
use std::sync::Mutex;
use tokio::runtime::{Runtime, TaskExecutor};

pub struct Boomslang {
    runtime: Mutex<Runtime>,
    executor: TaskExecutor,
}

impl Boomslang {
    fn spawn<F>(&self, future: F) -> Result<()>
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        self.executor.spawn(future);
        Ok(())
    }

    fn wait<F>(&self, future: F) -> Result<()>
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        let mut rt = self.runtime.lock().map_err(|e| Critical {
            message: format!("Failed to wait task due to runtime poisoning!\n{:?}", e),
        })?;
        rt.block_on(future).map_err(|_| Critical {
            message: "Unknown error occurred!".into(),
        })?;
        Ok(())
    }

    pub fn new() -> Result<Boomslang> {
        let rt = Runtime::new().map_err(|e| Critical {
            message: format!("Failed to start runtime!\n{:?}", e),
        })?;
        let executor = rt.executor();
        Ok(Boomslang {
            runtime: Mutex::new(rt),
            executor,
        })
    }

    pub fn run(&self, agent: Agent) -> Result<()> {
        match agent {
            Agent::Executor(executor) => self.execute(executor)?,
            _ => {
                return Err(Critical {
                    message: "Attempted to run unimplemented agent type!".into(),
                })
            }
        }
        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        self.add_signal_hooks()?;
        Ok(())
    }
}
