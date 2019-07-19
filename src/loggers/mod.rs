use crate::prelude::*;
use crate::runtime::Message;
use parking_lot::Mutex;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::*;

mod file;
mod stdout;

impl_module! {
    Logger, LoggerSink: {
        Stdout, stdout => { }
        File, file => { path: PathBuf }
    } => {
        log(m: Message) -> Result<()>
    }
}

impl Logger {
    pub fn init(&self) -> Result<()> {
        for_match!(self: Logger [Stdout, File] |inner| (
            let receiver = inner.receiver.lock().take().ok_or_else(|| Critical { message: "Can only be initialized once!".into() })?;
            let inner_send = inner.clone();
            spawn!(
                receiver.for_each(move |m: Message| {
                    inner_send.log(m).unwrap_or_default();
                    Ok(())
                })
            ))
        )
    }
}
