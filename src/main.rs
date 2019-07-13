mod agents;
mod args;
mod config;
mod error;
mod prelude;
mod runtime;

use args::Args;
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
    let f = std::fs::File::open(args.config)?;
    let config: config::BaseConfig = serde_yaml::from_reader(f)?;
    let agents: Vec<Agent> = config
        .agent
        .into_iter()
        .map(|(_, c)| c.into_agent())
        .collect();
    for agent in agents.into_iter() {
        app.run(agent)?;
    }
    app.start()
}
