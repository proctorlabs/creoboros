use chrono::Local;
use log::{Level, Log, Metadata, Record, SetLoggerError};
use std::collections::BTreeMap;
use unstructured::Document;

struct LogForwarder {
    level: Level,
}

impl Log for LogForwarder {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let logger = crate::runtime::BOOMSLANG.get_logger(record.target());
            if let Some(l) = logger {
                let mut log: BTreeMap<Document, Document> = BTreeMap::new();
                log.insert("timestamp".into(), Local::now().to_rfc3339().into());
                log.insert("log".into(), format!("{}", record.args()).into());
                log.insert("level".into(), record.level().to_string().into());
                l.send(crate::runtime::Message::Log { log: log.into() });
            } else {
                let level_string = record.level().to_string();
                println!(
                    "{} |{:<5}| {}",
                    Local::now().format("%H:%M:%S.%3f"),
                    level_string,
                    record.args()
                );
            }
        }
    }

    fn flush(&self) {
        println!("flush!");
    }
}

pub fn init(level: Level) -> Result<(), SetLoggerError> {
    let logger = LogForwarder { level };
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}
