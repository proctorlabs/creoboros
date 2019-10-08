use super::*;

use async_std::sync::Mutex;
use std::fs::File as StdFile;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, new)]
pub struct File {
    name: String,
    path: PathBuf,
    formatter: Formatters,
    #[new(default)]
    file: Mutex<Option<StdFile>>,
}

impl ModuleExt for File {
    fn initialize(&mut self, _: &Sender<Message>) -> Result<()> {
        task::block_on(async {
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(self.path.clone())?;
            let mut flock = self.file.lock().await;
            *flock = Some(file);
            info!("Logger initialized!" logger: self.name => self.name);
            Ok(())
        })
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, m: Message) -> Result<()> {
        task::block_on(async {
            match m {
                Log { log } => {
                    let mut w = Vec::new();
                    let msg = self.formatter.format(log);
                    writeln!(w, "{}", msg).unwrap_or_default();
                    let mut file = self.file.lock().await;
                    if let Some(f) = &mut *file {
                        f.write_all(&w)?;
                    } else {
                        warn!("No handle available to write to file!" logger: self.name);
                    }
                }
                Unit => {}
            };
            Ok(())
        })
    }

    fn priority(&self) -> u16 {
        1
    }
}
