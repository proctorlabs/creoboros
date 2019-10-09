#![recursion_limit = "128"]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate derive_new;

#[macro_use]
mod macros;

mod actions;
mod args;
mod config;
mod error;
mod format;
mod modules;
mod prelude;
mod runtime;
mod templates;

use args::Args;
use config::Builder;
use prelude::*;
use runtime::RT;

fn main() -> Result<()> {
    let res = task::block_on(async {
        let args = Args::new();

        let config = match args.inline {
            None => config::BaseConfig::load_file(args.config)?,
            Some(s) => config::BaseConfig::load_str(&s)?,
        };

        templates::context_set_value(&config.vars)?;

        for action in config.actions.build()?.into_iter() {
            RT.register_action(action).await?;
        }

        for agent in config.modules.build()?.into_iter() {
            RT.register(agent).await?;
        }

        RT.start()
    });
    if let Err(e) = res {
        eprintln!("Failure executing!");
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}
