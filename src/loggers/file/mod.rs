use super::*;
use std::fs::OpenOptions;
use std::io::Write;
use tokio::fs::File as TFile;

impl File {
    fn init(&self) {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.path.clone())
            .unwrap();
        let mut flock = self.file.lock();
        *flock = Some(TFile::from_std(file));
        info!("Logger initialized!" logger: self.name => self.name)
    }
}

impl LoggerSink for File {
    fn log(&self, m: Message) -> Result<()> {
        match m {
            Log { log } => {
                let mut w = Vec::new();
                writeln!(w, "{}", serde_json::to_string(&log).unwrap_or_default())
                    .unwrap_or_default();
                let mut file = self.file.lock();
                if let Some(f) = &mut *file {
                    f.write_all(&w).unwrap();
                } else {
                    warn!("No handle available to write to file!" logger: self.name);
                }
            }
            Init => {
                self.init();
            }
        };
        Ok(())
    }
}
