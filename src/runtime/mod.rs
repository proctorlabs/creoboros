use crate::modules::*;
use crate::prelude::*;
pub use message::{Message, Message::*};
use parking_lot::Mutex;
use std::collections::HashMap;

mod message;
mod signals;

lazy_static! {
    pub static ref CERBERUS: Cerberus = {
        Cerberus {
            modules: Default::default(),
        }
    };
    pub static ref STDOUT: crate::modules::Stdout =
        { crate::modules::Stdout::new("Default Logger".into()) };
}

pub struct Cerberus {
    modules: Mutex<HashMap<String, Box<dyn DynamicModule>>>,
}

impl Cerberus {
    pub fn register<T: Into<Box<dyn DynamicModule>>>(&self, module: T) -> Result<()> {
        let m: Box<dyn DynamicModule> = module.into();
        m.initialize()?;
        let mut map = self.modules.lock();
        map.insert(m.name(), m);
        Ok(())
    }

    pub fn send(&self, target: &str, message: Message) {
        if let Some(m) = self.modules.lock().get(target) {
            m.send(message).unwrap_or_else(|_| {
                crate::modules::Stdout::write(
                    &format!("Could not write to the specified target {}", target),
                    "CRIT",
                );
            });
        } else {
            STDOUT.handle(message).unwrap_or_default();
        }
    }

    pub fn start(&self) -> Result<()> {
        self.add_signal_hooks()
    }
}
