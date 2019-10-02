use crate::prelude::*;
use crate::runtime::Message;
use parking_lot::Mutex;
use std::fs::File as TFile;
use std::path::PathBuf;
use std::sync::Arc;

mod file;
mod stdout;

impl_module! {
    Logger, LoggerSink: {
        Stdout, stdout => { }
        File, file => { path: PathBuf, file: Mutex<Option<TFile>> }
    } => {
        log(m: Message) -> Result<()>
    }
}

impl Logger {
    pub fn init(&self) -> Result<()> {
        for_match!(self: Logger [Stdout, File] |inner| (
            let receiver = inner.receiver.clone();
            self.send(Init);
            capture!(inner:inner
                {
                    task::spawn(async move {
                        loop {
                            let m = receiver.recv().unwrap();
                            inner.log(m).unwrap_or_default();
                        }
                    });
                }
            )
        ));
        Ok(())
    }
}
