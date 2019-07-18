use crate::loggers::Logger;
use crate::prelude::*;
pub use message::{Message, Message::*};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::runtime::{Runtime, TaskExecutor};
use tokio::sync::mpsc::*;

lazy_static! {
    pub static ref BOOMSLANG: Boomslang = {
        let rt = Runtime::new()
            .map_err(|e| Critical {
                message: format!("Failed to start runtime!\n{:?}", e),
            })
            .unwrap();
        let executor = rt.executor();
        let chan = unbounded_channel();
        let msg_proc = chan
            .1
            .for_each(|m: Message| {
                match m {
                    Log { log } => info!("{}", serde_json::to_string(&log).unwrap_or_default()),
                };
                Ok(())
            })
            .map(|_| ())
            .map_err(|_| ());
        executor.spawn(msg_proc);
        Boomslang {
            runtime: Mutex::new(rt),
            executor,
            loggers: Default::default(),
            sender: chan.0,
        }
    };
}

pub type Spawnable = Box<dyn Future<Item = (), Error = ()> + Send>;

mod message;
mod signals;

pub struct Boomslang {
    runtime: Mutex<Runtime>,
    executor: TaskExecutor,
    loggers: Mutex<HashMap<String, Logger>>,
    pub(crate) sender: UnboundedSender<Message>,
}

impl Boomslang {
    pub fn spawn<F>(&self, future: F) -> Result<()>
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        self.executor.spawn(future);
        Ok(())
    }

    pub fn sender(&self) -> UnboundedSender<Message> {
        self.sender.clone()
    }

    pub fn register_logger(&self, logger: Logger) -> Result<()> {
        let mut map = self.loggers.lock().unwrap();
        map.insert(logger.get_name(), logger);
        Ok(())
    }

    fn wait<F>(&self, future: F) -> Result<()>
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        let mut rt = self.runtime.lock().map_err(|e| Critical {
            message: format!("Failed to wait task due to runtime poisoning!\n{:?}", e),
        })?;
        rt.block_on(future).map_err(|e| Critical {
            message: format!("Unknown error: {:?}", e),
        })?;
        Ok(())
    }

    pub fn run(&self, agent: Agent) -> Result<()> {
        agent.execute()
    }

    pub fn start(&self) -> Result<()> {
        self.add_signal_hooks()
    }
}
