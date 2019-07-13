use crate::agents::Agent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseConfig {
    #[serde(flatten)]
    pub agent: HashMap<String, AgentConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum AgentConfig {
    Exec { command: String },
}

impl AgentConfig {
    pub fn into_agent(self) -> Agent {
        match self {
            AgentConfig::Exec { command } => Agent::executor(command),
        }
    }
}
