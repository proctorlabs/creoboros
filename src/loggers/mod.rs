use crate::prelude::*;
use crate::runtime::Message;
use parking_lot::Mutex;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::*;
use tokio::fs::File as TFile;

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
            let receiver = inner.receiver.lock().take().ok_or_else(|| Critical { message: "Can only be initialized once!".into() })?;
            self.send(Init);
            capture!(inner:inner
                {
                    spawn!(
                        receiver.for_each(move |m: Message| {
                            inner.log(m).unwrap_or_default();
                            Ok(())
                        })
                    )
                }
            )
        ))
    }
}
