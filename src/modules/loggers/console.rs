use super::*;

#[derive(Debug, new)]
pub struct Console {
    name: String,
    formatter: Formatters,
}

impl Console {
    pub fn log(&self, msg: &str) {
        println!("{}", msg)
    }
}

impl ModuleExt for Console {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, m: Message) -> Result<()> {
        match m {
            Log { log } => {
                let msg = self.formatter.format(log);
                self.log(&msg);
            }
            Unit => info!("Log test..." logger: self.name => self.name),
        };
        Ok(())
    }
}
