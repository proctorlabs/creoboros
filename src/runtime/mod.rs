use crate::actions::*;
use crate::format::Formatters;
use crate::modules::*;
use crate::prelude::*;
use async_std::sync::Mutex;
use futures_channel::mpsc::unbounded;
pub use message::{Message, Message::*};
use std::collections::HashMap;

mod message;
mod signals;

type DelegatedMessage = (String, Message);

lazy_static! {
    pub static ref CONSOLE: Console = Console::new("default_logger".into(), Formatters::default());
    pub static ref RT: Creoboros = Creoboros::new();
}

#[derive(Debug)]
pub struct Creoboros {
    modules: Mutex<HashMap<String, Box<dyn DynamicModule>>>,
    actions: Mutex<HashMap<String, Action>>,
    sender: Sender<DelegatedMessage>,
}

impl Creoboros {
    fn new() -> Self {
        let (sender, mut r) = unbounded::<DelegatedMessage>();
        task::spawn(async move {
            while let Some((target, msg)) = r.next().await {
                if let Some(m) = RT.modules.lock().await.get(&target) {
                    m.send(msg).unwrap_or_else(|_| {
                        CONSOLE.log(&format!(
                            "Could not write to the specified target {}",
                            target
                        ));
                    });
                } else {
                    CONSOLE.handle(msg).unwrap_or_default();
                }
            }
        });
        Creoboros {
            modules: Default::default(),
            actions: Default::default(),
            sender,
        }
    }

    pub async fn register<T: Into<Box<dyn DynamicModule>>>(&self, module: T) -> Result<()> {
        let mut m: Box<dyn DynamicModule> = module.into();
        let name = m.name();
        info!("Initializing agent {}"[name] agent: name);
        m.initialize()?;
        let mut map = self.modules.lock().await;
        map.insert(m.name(), m);
        drop(map);
        info!("Agent {} fully initialized!"[name] agent: name);
        Ok(())
    }

    pub async fn register_action(&self, action: Action) -> Result<()> {
        let mut map = self.actions.lock().await;
        map.insert(action.name(), action);
        Ok(())
    }

    pub async fn execute(&self, logger: String, action: String) -> Result<()> {
        let out = logger.clone();
        info!("Executing action '{}'..."[action] => out);
        let res = {
            let map = self.actions.lock().await;
            let action = map.get(&action).cloned();
            drop(map);
            if let Some(action) = action {
                action.execute(logger)
            } else {
                Err(Critical {
                    message: "Attempted to execute non-existant action!".into(),
                })
            }
        };
        if let Err(e) = &res {
            warn!("Failed to execute action! {}"[e] => out);
        } else {
            info!("Execution of action '{}' complete"[action] => out);
        }
        res
    }

    pub fn send(&self, target: &str, message: Message) {
        self.sender
            .unbounded_send((target.into(), message))
            .unwrap_or_default();
    }

    pub fn start(&self) -> Result<()> {
        self.add_signal_hooks()
    }
}
