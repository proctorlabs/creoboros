use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Boomslang", rename_all = "kebab_case")]
pub struct Args {
    /// Config file to load
    #[structopt(
        short,
        long,
        help = "Config file location",
        default_value = "/etc/boomslang.yml"
    )]
    pub config: PathBuf,

    /// Inline configuration
    #[structopt(long, env = "INIT_YAML", help = "Inline YAML configuration")]
    pub inline: Option<String>,

    /// Operation mode
    #[structopt(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "command", rename_all = "kebab_case")]
pub enum Command {
    /// Start Boomslang (default)
    Run,
}

impl Args {
    pub fn new() -> Self {
        Args::from_args()
    }
}
