#![recursion_limit = "128"]
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
mod macros;

mod agents;
mod args;
mod config;
mod error;
mod loggers;
mod prelude;
mod runtime;

use args::Args;
use config::Builder;
use prelude::*;
use runtime::BOOMSLANG;

fn main() -> Result<()> {
    let args = Args::new();
    loggers::init(args.log_level).map_err(|e| Critical {
        message: format!("Failed to initialize logging!\n{:?}", e),
    })?;

    let config = match args.inline {
        None => config::BaseConfig::load_file(args.config)?,
        Some(s) => config::BaseConfig::load_str(&s)?,
    };

    for logger in config.loggers.build().into_iter() {
        BOOMSLANG.register_logger(logger)?;
    }

    for agent in config.agents.build().into_iter() {
        BOOMSLANG.run(agent)?;
    }
    BOOMSLANG.start()
}
