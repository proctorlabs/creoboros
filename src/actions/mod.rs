use crate::prelude::*;
use std::fmt::Debug;

mod file_template;
mod run;

pub use file_template::FileTemplate;
pub use run::Run;

pub trait ActionImpl: Send + Sync + Debug {
    fn execute(&self, logger: String) -> Result<()>;
    fn name(&self) -> String;
}

#[derive(From, Debug, Clone)]
pub struct Action(Arc<dyn ActionImpl + Send + Sync>);

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
        Action(Arc::new(a))
    }
}
