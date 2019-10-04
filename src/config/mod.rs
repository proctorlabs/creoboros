mod builders;
mod definition;
mod misc;

use crate::prelude::*;
use crate::templates::*;
use misc::OneOrMany;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub use builders::*;
pub use definition::*;

impl BaseConfig {
    pub fn load_file(file: PathBuf) -> Result<Self> {
        let f = std::fs::File::open(file)?;
        Ok(serde_yaml::from_reader(f)?)
    }

    pub fn load_str(conf: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(conf)?)
    }
}
