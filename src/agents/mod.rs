use crate::prelude::*;
use crate::runtime::Message;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::*;
use crate::config::Policy;

mod service;
mod timer;

impl_module! {
    Agent, RunnableAgent: {
        Service, service => { command: String, args: Vec<String>, logger: String, policy: Policy }
        Timer, timer => { interval: Duration, logger: String }
    } => {
        execute() -> Result<()>
    }
}
