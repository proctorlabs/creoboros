use crate::agents::Agent;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseConfig {
    pub agents: HashMap<String, AgentConfig>,
}

impl BaseConfig {
    pub fn load_file(file: PathBuf) -> Result<Self> {
        let f = std::fs::File::open(file)?;
        Ok(serde_yaml::from_reader(f)?)
    }

    pub fn load_str(conf: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(conf)?)
    }
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
