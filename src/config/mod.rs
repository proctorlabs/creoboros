use crate::agents::Agent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseConfig {
    #[serde(flatten)]
    pub agents: HashMap<String, AgentConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum AgentConfig {
    Exec { command: String, args: Vec<String> },
    Script { shell: String, script: String },
}

impl AgentConfig {
    pub fn into_agent(self, name: String) -> Agent {
        match self {
            AgentConfig::Exec { command, args } => Agent::executor(name, command, args),
            AgentConfig::Script { shell, script } => {
                Agent::executor(name, shell, vec!["-c".into(), script])
            }
        }
    }
}
