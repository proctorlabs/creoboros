use crate::actions::*;
use crate::modules::*;
use crate::prelude::*;
use crossbeam_channel::{unbounded, Sender};
pub use message::{Message, Message::*};
use parking_lot::Mutex;
use std::collections::HashMap;

mod message;
mod signals;

type DelegatedMessage = (String, Message);

lazy_static! {
    pub static ref STDOUT: Stdout = Stdout::new("default".into());
    pub static ref CERBERUS: Cerberus = Cerberus::new();
}

pub struct Cerberus {
    modules: Mutex<HashMap<String, Box<dyn DynamicModule>>>,
    actions: Mutex<HashMap<String, Action>>,
    sender: Sender<DelegatedMessage>,
}

impl Cerberus {
    fn new() -> Self {
        let (sender, r) = unbounded::<DelegatedMessage>();
        task::spawn(async move {
            while let Ok((target, msg)) = r.recv() {
                if let Some(m) = CERBERUS.modules.lock().get(&target) {
                    m.send(msg).unwrap_or_else(|_| {
                        Stdout::write(
                            &format!("Could not write to the specified target {}", target),
                            "CRIT",
                        );
                    });
                } else {
                    STDOUT.handle(msg).unwrap_or_default();
                }
            }
        });
        Cerberus {
            modules: Default::default(),
            actions: Default::default(),
            sender,
        }
    }

    pub fn register<T: Into<Box<dyn DynamicModule>>>(&self, module: T) -> Result<()> {
        let m: Box<dyn DynamicModule> = module.into();
        m.initialize()?;
        let mut map = self.modules.lock();
        map.insert(m.name(), m);
        Ok(())
    }

    pub fn register_action(&self, action: Action) -> Result<()> {
        let mut map = self.actions.lock();
        map.insert(action.name(), action);
        Ok(())
    }

    pub fn send(&self, target: &str, message: Message) {
        self.sender
            .send((target.into(), message))
            .unwrap_or_default();
    }

    pub fn start(&self) -> Result<()> {
        self.add_signal_hooks()
    }
}
