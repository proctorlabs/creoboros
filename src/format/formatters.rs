use super::*;

#[derive(Debug, Clone)]
pub struct Plain;

impl Formatter for Plain {
    fn format(&self, log: Document) -> String {
        log["log"].to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Standard;

impl Formatter for Standard {
    fn format(&self, log: Document) -> String {
        let agent_str = if log["agent"].is_string() {
            format!(" [{}]", log["agent"])
        } else {
            "".into()
        };
        format!(
            "{} |{:<4}|{}: {}",
            chrono::Local::now().format("%H:%M:%S.%3f"),
            &log["level"],
            agent_str,
            &log["log"]
        )
    }
}

#[derive(Debug, Clone)]
pub struct Json;

impl Formatter for Json {
    fn format(&self, log: Document) -> String {
        serde_json::to_string(&log).unwrap_or_default()
    }
}
