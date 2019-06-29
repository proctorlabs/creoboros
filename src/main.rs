mod agents;
mod config;
mod error;
mod prelude;
mod runtime;

use config::Args;
use prelude::*;
use runtime::Boomslang;

#[macro_use]
extern crate log;

fn main() -> Result<()> {
    let args = Args::new();
    simple_logger::init_with_level(args.log_level).map_err(|e| Critical {
        message: format!("Failed to initialize logging!\n{:?}", e),
    })?;
    let app = Boomslang::new()?;
    app.run(Agent::executor(args.script))?;
    app.start()
}
