use crate::prelude::*;

mod agents;
mod loggers;

pub use agents::*;
pub use loggers::*;

use async_std::sync::RwLock;
use std::fmt::Debug;

pub trait ModuleExt: Send + Sync + Debug {
    #[inline]
    fn initialize(&mut self, _: &Sender<Message>) -> Result<()> {
        Ok(())
    }

    fn name(&self) -> String;

    fn handle(&self, _: Message) -> Result<()> {
        Ok(())
    }

    fn priority(&self) -> u16 {
        100
    }
}

pub trait DynamicModule: Send + Sync + Debug {
    fn name(&self) -> String;
    fn send(&self, message: Message) -> Result<()>;
    fn initialize(&mut self) -> Result<()>;
    fn priority(&self) -> u16;
}

#[derive(Debug)]
pub struct Module<T: ModuleExt> {
    module: Arc<RwLock<T>>,
    sender: Sender<Message>,
    receiver: Option<Receiver<Message>>,
}

impl<T: 'static + ModuleExt> DynamicModule for Module<T> {
    #[inline]
    fn name(&self) -> String {
        task::block_on(async { self.module.read().await.name() })
    }

    #[inline]
    fn send(&self, message: Message) -> Result<()> {
        self.sender.unbounded_send(message).unwrap_or_default();
        Ok(())
    }

    fn initialize(&mut self) -> Result<()> {
        task::block_on(async {
            self.module.write().await.initialize(&self.sender)?;
            let module = self.module.clone();
            if let Some(mut receiver) = self.receiver.take() {
                task::spawn(async move {
                    while let Some(message) = receiver.next().await {
                        module.read().await.handle(message)?;
                    }
                    Ok::<(), AppError>(())
                });
            }
            Ok(())
        })
    }

    fn priority(&self) -> u16 {
        task::block_on(async { self.module.read().await.priority() })
    }
}

impl<T: ModuleExt> From<T> for Module<T> {
    fn from(module: T) -> Self {
        let (sender, receiver) = futures_channel::mpsc::unbounded();
        let result: Module<T> = Module {
            module: Arc::new(RwLock::new(module)),
            sender,
            receiver: Some(receiver),
        };
        result
    }
}

impl<T: 'static + ModuleExt> From<Module<T>> for Box<dyn DynamicModule> {
    fn from(module: Module<T>) -> Self {
        Box::new(module)
    }
}
