use crate::agents::Agent;
use crate::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum AgentConfig {
    Exec {
        command: String,
        args: Vec<String>,
    },
    Script {
        shell: String,
        script: String,
    },
    Timer {
        #[serde(with = "serde_humanize_rs")]
        interval: Duration,
    },
}

impl AgentConfig {
    pub fn into_agent(self, name: String) -> Agent {
        match self {
            AgentConfig::Exec { command, args } => Agent::executor(name, command, args),
            AgentConfig::Script { shell, script } => {
                Agent::executor(name, shell, vec!["-c".into(), script])
            }
            AgentConfig::Timer { interval } => Agent::timer(name, interval),
        }
    }
}
