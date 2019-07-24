use super::*;
use std::fs::OpenOptions;
use std::io::Write;

impl LoggerSink for File {
    fn log(&self, m: Message) -> Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("out.log")
            .unwrap();
        let mut w = Vec::new();
        match m {
            Log { log } => writeln!(w, "{}", serde_json::to_string(&log).unwrap_or_default()),
        }
        .unwrap_or_default();
        file.write_all(&w).unwrap();
        Ok(())
    }
}
