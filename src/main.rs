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
mod prelude;
mod runtime;

use args::Args;
use prelude::*;
use runtime::BOOMSLANG;

fn main() -> Result<()> {
    let args = Args::new();
    simple_logger::init_with_level(args.log_level).map_err(|e| Critical {
        message: format!("Failed to initialize logging!\n{:?}", e),
    })?;

    let config = match args.inline {
        None => config::BaseConfig::load_file(args.config)?,
        Some(s) => config::BaseConfig::load_str(&s)?,
    };

    let agents: Vec<Agent> = config
        .agents
        .into_iter()
        .map(|(n, c)| c.into_agent(n))
        .collect();

    for agent in agents.into_iter() {
        BOOMSLANG.run(agent)?;
    }
    BOOMSLANG.start()
}
