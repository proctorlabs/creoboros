use super::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseConfig {
    #[serde(default)]
    pub vars: Document,
    #[serde(default)]
    pub actions: HashMap<String, ActionConfig>,
    #[serde(default)]
    pub loggers: HashMap<String, LoggerConfig>,
    pub agents: HashMap<String, AgentConfig>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum Policy {
    Restart {
        #[serde(default = "default_delay")]
        delay: u64,
    },
    Nothing,
}

impl Default for Policy {
    fn default() -> Self {
        Policy::Restart {
            delay: default_delay(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ActionConfig {
    Run {
        script: ConfigTemplate,
        #[serde(default = "default_shell")]
        shell: ConfigTemplate,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum LoggerConfig {
    File { path: PathBuf },
    Stdout,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum AgentConfig {
    Exec {
        #[serde(default = "default_logger")]
        logger: String,
        command: ConfigTemplate,
        #[serde(default)]
        args: OneOrMany<ConfigTemplate>,
        #[serde(default)]
        policy: Policy,
    },
    Script {
        #[serde(default = "default_logger")]
        logger: String,
        #[serde(default = "default_shell")]
        shell: ConfigTemplate,
        script: ConfigTemplate,
        #[serde(default)]
        policy: Policy,
    },
    Timer {
        #[serde(default = "default_logger")]
        logger: String,
        #[serde(deserialize_with = "serde_humanize_rs::deserialize")]
        interval: Duration,
    },
}

fn default_logger() -> String {
    "default".to_string()
}

fn default_shell() -> ConfigTemplate {
    "/bin/bash".into()
}

fn default_delay() -> u64 {
    10
}
