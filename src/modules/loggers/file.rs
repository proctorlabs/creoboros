use super::*;

use parking_lot::Mutex;
use std::fs::File as StdFile;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub struct File {
    name: String,
    path: PathBuf,
    file: Mutex<Option<StdFile>>,
}

impl File {
    pub fn new(name: String, path: PathBuf) -> Self {
        File {
            name,
            path,
            file: Default::default(),
        }
    }
}

impl ModuleExt for File {
    fn initialize(&self, _: &Sender<Message>) -> Result<()> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.path.clone())?;
        let mut flock = self.file.lock();
        *flock = Some(file);
        info!("Logger initialized!" logger: self.name => self.name);
        Ok(())
    }
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, m: Message) -> Result<()> {
        match m {
            Log { log } => {
                let mut w = Vec::new();
                writeln!(w, "{}", serde_json::to_string(&log).unwrap_or_default())
                    .unwrap_or_default();
                let mut file = self.file.lock();
                if let Some(f) = &mut *file {
                    f.write_all(&w)?;
                } else {
                    warn!("No handle available to write to file!" logger: self.name);
                }
            }
            Unit => {}
        };
        Ok(())
    }
}
