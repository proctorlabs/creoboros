use super::*;

#[derive(Debug)]
pub struct Plain;

impl Formatter for Plain {
    fn format(&self, log: Document) -> String {
        log["log"].to_string()
    }
}

#[derive(Debug)]
pub struct Standard;

impl Formatter for Standard {
    fn format(&self, log: Document) -> String {
        format!(
            "{} |{:<4}|: {}",
            chrono::Local::now().format("%H:%M:%S.%3f"),
            &log["level"],
            &log["log"]
        )
    }
}

#[derive(Debug)]
pub struct Json;

impl Formatter for Json {
    fn format(&self, log: Document) -> String {
        serde_json::to_string(&log).unwrap_or_default()
    }
}
