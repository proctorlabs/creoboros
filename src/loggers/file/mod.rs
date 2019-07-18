use super::*;

impl LoggerSink for File {
    fn log(self) -> Result<()> {
        Ok(())
    }
}
