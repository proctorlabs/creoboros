use crate::config::Policy;
use crate::prelude::*;
use crate::runtime::Message;
use std::sync::Arc;
use std::time::Duration;

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
