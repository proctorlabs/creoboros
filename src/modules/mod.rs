use crate::prelude::*;

mod agents;
mod loggers;

pub use agents::*;
pub use loggers::*;

use std::fmt::Debug;

pub trait ModuleExt: Send + Sync + Debug {
    #[inline]
    fn initialize(&self, _: &Sender<Message>) -> Result<()> {
        Ok(())
    }

    fn name(&self) -> String;

    fn handle(&self, message: Message) -> Result<()>;
}

pub trait DynamicModule: Send + Sync + Debug {
    fn name(&self) -> String;
    fn send(&self, message: Message) -> Result<()>;
    fn initialize(&self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct Module<T: ModuleExt> {
    module: Arc<T>,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl<T: 'static + ModuleExt> DynamicModule for Module<T> {
    #[inline]
    fn name(&self) -> String {
        self.module.name()
    }

    #[inline]
    fn send(&self, message: Message) -> Result<()> {
        Ok(self.sender.send(message)?)
    }

    fn initialize(&self) -> Result<()> {
        self.module.initialize(&self.sender)?;
        let receiver = self.receiver.clone();
        let module = self.module.clone();
        task::spawn(async move {
            while let Ok(message) = receiver.recv() {
                module.handle(message)?;
            }
            Ok::<(), AppError>(())
        });
        Ok(())
    }
}

impl<T: ModuleExt> From<T> for Module<T> {
    fn from(module: T) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let result: Module<T> = Module {
            module: Arc::new(module),
            sender,
            receiver,
        };
        result
    }
}

impl<T: 'static + ModuleExt> From<Module<T>> for Box<dyn DynamicModule> {
    fn from(module: Module<T>) -> Self {
        Box::new(module)
    }
}
