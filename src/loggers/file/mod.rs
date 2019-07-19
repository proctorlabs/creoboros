use super::*;

impl LoggerSink for File {
    fn log(&self, m: Message) -> Result<()> {
        match m {
            Log { log } => info!("{}", log["log"]),
        };
        Ok(())
    }
}
