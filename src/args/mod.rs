use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Cerberus", rename_all = "kebab_case")]
pub struct Args {
    /// Config file to load
    #[structopt(
        short,
        long,
        help = "Config file location",
        default_value = "/etc/cerberus.yml"
    )]
    pub config: PathBuf,

    /// Inline configuration
    #[structopt(long, env = "INIT_YAML", help = "Inline YAML configuration")]
    pub inline: Option<String>,
}

impl Args {
    pub fn new() -> Self {
        Args::from_args()
    }
}
