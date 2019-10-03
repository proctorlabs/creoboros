use super::*;

pub struct Stdout {
    name: String,
}

impl Stdout {
    pub fn new(name: String) -> Self {
        Stdout { name }
    }

    pub fn write(msg: &str, level: &str) {
        println!(
            "{} |{:<4}|: {}",
            chrono::Local::now().format("%H:%M:%S.%3f"),
            level,
            msg
        );
    }
}

impl ModuleExt for Stdout {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, m: Message) -> Result<()> {
        match m {
            Log { log } => Stdout::write(&log["log"].to_string(), &log["level"].to_string()),
            Init => info!("Logger initialized!" logger: self.name => self.name),
        };
        Ok(())
    }
}
