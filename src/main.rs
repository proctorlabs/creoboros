#![recursion_limit = "128"]
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

mod actions;
mod args;
mod config;
mod error;
mod modules;
mod prelude;
mod runtime;
mod templates;

use args::Args;
use config::Builder;
use prelude::*;
use runtime::CERBERUS;

fn main() -> Result<()> {
    task::block_on(async {
        let args = Args::new();

        let config = match args.inline {
            None => config::BaseConfig::load_file(args.config)?,
            Some(s) => config::BaseConfig::load_str(&s)?,
        };

        templates::context_set_value(&config.vars)?;

        for action in config.actions.build().into_iter() {
            CERBERUS.register_action(action)?;
        }

        for logger in config.loggers.build().into_iter() {
            CERBERUS.register(logger)?;
        }

        for agent in config.agents.build().into_iter() {
            CERBERUS.register(agent)?;
        }

        CERBERUS.start()
    })
}
