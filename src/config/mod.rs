use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseConfig {
    pub agent: Vec<AgentConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgentConfig {
    pub command: String,
}
