use crate::agents::Agent;
use crate::loggers::Logger;
use crate::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

fn default_logger() -> String {
    "default".to_string()
}

fn default_shell() -> String {
    "/bin/bash".to_string()
}

pub trait Builder<T> {
    fn build(self) -> T;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseConfig {
    #[serde(default)]
    pub loggers: HashMap<String, LoggerConfig>,
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
pub enum LoggerConfig {
    File { path: PathBuf },
    Stdout,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum AgentConfig {
    Exec {
        #[serde(default = "default_logger")]
        logger: String,
        command: String,
        #[serde(default)]
        args: Vec<String>,
    },
    Script {
        #[serde(default = "default_logger")]
        logger: String,
        #[serde(default = "default_shell")]
        shell: String,
        script: String,
    },
    Timer {
        #[serde(default = "default_logger")]
        logger: String,
        #[serde(with = "serde_humanize_rs")]
        interval: Duration,
    },
}

impl Builder<Vec<Logger>> for HashMap<String, LoggerConfig> {
    fn build(self) -> Vec<Logger> {
        self.into_iter()
            .map(|(n, c)| match c {
                LoggerConfig::Stdout => Logger::stdout(n),
                LoggerConfig::File { path } => Logger::file(n, path),
            })
            .collect()
    }
}

impl Builder<Vec<Agent>> for HashMap<String, AgentConfig> {
    fn build(self) -> Vec<Agent> {
        self.into_iter()
            .map(|(n, c)| match c {
                AgentConfig::Exec {
                    command,
                    args,
                    logger,
                } => Agent::service(n, command, args, logger),
                AgentConfig::Script {
                    shell,
                    script,
                    logger,
                } => Agent::service(n, shell, vec!["-c".into(), script], logger),
                AgentConfig::Timer { interval, logger } => Agent::timer(n, interval, logger),
            })
            .collect()
    }
}
