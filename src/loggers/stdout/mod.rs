use super::*;

impl Stdout {
    pub fn write(msg: &str, level: &str) {
        println!(
            "{} |{:<4}|: {}",
            chrono::Local::now().format("%H:%M:%S.%3f"),
            level,
            msg
        );
    }
}

impl LoggerSink for Stdout {
    fn log(&self, m: Message) -> Result<()> {
        match m {
            Log { log } => Stdout::write(&log["log"].to_string(), &log["level"].to_string()),
        };
        Ok(())
    }
}
