use crate::loggers::*;
use crate::prelude::*;
pub use message::{Message, Message::*};
use parking_lot::Mutex;
use std::collections::HashMap;

mod message;
mod signals;

lazy_static! {
    pub static ref CERBERUS: Cerberus = {
        // use crossbeam_channel::*;
        // let (_, r) = unbounded::<Message>();
        // async_std::task::spawn(async move {
        //     loop {
        //         let m = r.recv().unwrap();
        //         info!("Master received message {:?}"[m]);
        //     }
        // });
        Cerberus {
            loggers: Default::default(),
            agents: Default::default(),
        }
    };
}

pub struct Cerberus {
    loggers: Mutex<HashMap<String, Logger>>,
    agents: Mutex<HashMap<String, Agent>>,
}

impl Cerberus {
    pub fn register_logger(&self, logger: Logger) -> Result<()> {
        let mut map = self.loggers.lock();
        logger.init()?;
        map.insert(logger.get_name(), logger);
        Ok(())
    }

    pub fn get_logger(&self, logger: &str) -> Option<Logger> {
        self.loggers.lock().get(logger).cloned()
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
