use crate::prelude::*;

mod run;

pub use run::Run;

pub trait ActionImpl: Send + Sync {
    fn execute(&self, logger: String) -> Result<()>;
    fn name(&self) -> String;
}

pub struct Action(Box<dyn ActionImpl + Send + Sync>);

impl Action {
    pub fn execute(&self, logger: String) -> Result<()> {
        self.0.execute(logger)
    }

    pub fn name(&self) -> String {
        self.0.name()
    }
}

impl<T: 'static + ActionImpl> From<T> for Action {
    fn from(a: T) -> Self {
        Action(Box::new(a))
    }
}
