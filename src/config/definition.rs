use super::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct BaseConfig {
    #[serde(default)]
    pub vars: Document,
    #[serde(default)]
    pub actions: HashMap<String, ActionConfig>,
    pub modules: HashMap<String, ModuleConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum ActionConfig {
    Run {
        script: ConfigTemplate,
        #[serde(default = "default_shell")]
        shell: ConfigTemplate,
    },
    FileTemplate {
        template: PathBuf,
        target: PathBuf,
    },
    Action {
        action: OneOrMany<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum ModuleConfig {
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
        timer: OneOrMany<String>,
    },

    RegexParser {
        #[serde(flatten)]
        pattern: ParserFormat,
        forward_to: String,
    },

    FileLogger {
        path: PathBuf,
        #[serde(default)]
        format: OutputFormat,
    },

    Console {
        console: ConsoleOutput,
        #[serde(default)]
        format: OutputFormat,
    },

    Start {
        start: OneOrMany<String>,
        #[serde(default = "default_logger")]
        logger: String,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ConsoleOutput {
    Stdout,
    Stderr,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Standard,
    Plain,
    Json,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum ParserFormat {
    Pattern {
        pattern: String,
    },
    BuiltIn {
        built_in_pattern: BuiltInParserFormat,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BuiltInParserFormat {
    Nginx,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Standard
    }
}

fn default_logger() -> String {
    "default_logger".to_string()
}

fn default_shell() -> ConfigTemplate {
    "/bin/bash".into()
}

fn default_delay() -> u64 {
    10_u64
}
