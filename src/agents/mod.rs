use crate::prelude::*;
use crate::runtime::Message;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::*;

mod executor;
mod timer;

impl_module! {
    Agent, RunnableAgent: {
        Service, service => { command: String, args: Vec<String>, logger: String }
        Timer, timer => { interval: Duration, logger: String }
    } => {
        execute() -> Result<()>
    }
}
