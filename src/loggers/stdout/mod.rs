use super::*;

impl LoggerSink for Stdout {
    fn log(&self, m: Message) -> Result<()> {
        match m {
            Log { log } => println!("{}", serde_json::to_string(&log).unwrap_or_default()),
        };
        Ok(())
    }
}
