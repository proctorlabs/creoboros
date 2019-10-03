use crate::modules::*;
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

fn default_delay() -> u64 {
    10
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseConfig {
    #[serde(default)]
    pub loggers: HashMap<String, LoggerConfig>,
    pub agents: HashMap<String, AgentConfig>,
}

#[derive(Debug, Deserialize, Clone)]
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
        #[serde(default)]
        policy: Policy,
    },
    Script {
        #[serde(default = "default_logger")]
        logger: String,
        #[serde(default = "default_shell")]
        shell: String,
        script: String,
        #[serde(default)]
        policy: Policy,
    },
    Timer {
        #[serde(default = "default_logger")]
        logger: String,
        #[serde(with = "serde_humanize_rs")]
        interval: Duration,
    },
}

pub trait Builder<T> {
    fn build(self) -> T;
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

impl Builder<Vec<Box<dyn DynamicModule>>> for HashMap<String, LoggerConfig> {
    fn build(self) -> Vec<Box<dyn DynamicModule>> {
        self.into_iter()
            .map(|(n, c)| match c {
                LoggerConfig::Stdout => Module::<Stdout>::from(Stdout::new(n)).into(),
                LoggerConfig::File { path } => Module::<File>::from(File::new(n, path)).into(),
            })
            .collect()
    }
}

impl Builder<Vec<Box<dyn DynamicModule>>> for HashMap<String, AgentConfig> {
    fn build(self) -> Vec<Box<dyn DynamicModule>> {
        self.into_iter()
            .map(|(n, c)| match c {
                AgentConfig::Exec {
                    command,
                    args,
                    logger,
                    policy,
                } => Module::<Process>::from(Process::new(n, command, args, logger, policy)).into(),
                AgentConfig::Script {
                    shell,
                    script,
                    logger,
                    policy,
                } => Module::<Process>::from(Process::new(
                    n,
                    shell,
                    vec!["-c".into(), script],
                    logger,
                    policy,
                ))
                .into(),
                AgentConfig::Timer { interval, logger } => {
                    Module::<Timer>::from(Timer::new(n, interval, logger)).into()
                }
            })
            .collect()
    }
}
