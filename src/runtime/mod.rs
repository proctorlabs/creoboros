use crate::loggers::*;
use crate::prelude::*;
pub use message::{Message, Message::*};
use parking_lot::Mutex;
use std::collections::HashMap;
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
        let (_, receiver) = unbounded_channel();
        let msg_proc = receiver
            .for_each(|m: Message| {
                info!("Master received message {:?}"[m]);
                Ok(())
            })
            .map(|_| ())
            .map_err(|_| ());
        executor.spawn(msg_proc);
        Boomslang {
            runtime: Mutex::new(rt),
            executor,
            loggers: Default::default(),
            agents: Default::default(),
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
    agents: Mutex<HashMap<String, Agent>>,
}

impl Boomslang {
    pub fn spawn<F>(&self, future: F) -> Result<()>
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        self.executor.spawn(future);
        Ok(())
    }

    pub fn register_logger(&self, logger: Logger) -> Result<()> {
        let mut map = self.loggers.lock();
        logger.init()?;
        map.insert(logger.get_name(), logger);
        Ok(())
    }

    pub fn get_logger(&self, logger: &str) -> Option<Logger> {
        self.loggers.lock().get(logger).cloned()
    }

    fn wait<F>(&self, future: F) -> Result<()>
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        let mut rt = self.runtime.lock();
        rt.block_on(future).map_err(|e| Critical {
            message: format!("Unknown error: {:?}", e),
        })?;
        Ok(())
    }

    pub fn run(&self, agent: Agent) -> Result<()> {
        let mut map = self.agents.lock();
        agent.execute()?;
        map.insert(agent.get_name(), agent);
        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        self.add_signal_hooks()
    }
}
