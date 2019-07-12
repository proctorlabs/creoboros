use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Boomslang", rename_all = "kebab_case")]
pub struct Args {
    #[structopt(
        short,
        long,
        help = "Config file location",
        default_value = "/etc/boomslang.yml"
    )]
    pub config: PathBuf,

    /// Filter to apply to input files
    #[structopt(short, long, help = "Logging level to use", default_value = "info")]
    pub log_level: log::Level,

    /// The command to run
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
