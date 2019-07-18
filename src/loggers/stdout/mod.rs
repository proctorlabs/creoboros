use super::*;

impl LoggerSink for Stdout {
    fn log(self) -> Result<()> {
        Ok(())
    }
}
