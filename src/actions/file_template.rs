use super::*;

use crate::templates::{CONTEXT, TEMPLAR};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use templar::Template;

#[derive(new, Default, Debug)]
pub struct FileTemplate {
    name: String,
    template: PathBuf,
    target: PathBuf,
}

impl ActionImpl for FileTemplate {
    fn execute(&self, _: String) -> Result<()> {
        let mut tpl = String::new();
        File::open(&self.template)?.read_to_string(&mut tpl)?;
        let prepared_tpl: Template = TEMPLAR.parse(&tpl)?;
        let result = prepared_tpl.render(&*CONTEXT)?;
        let mut target = OpenOptions::new()
            .write(true)
            .mode(0o644)
            .create_new(true)
            .open(&self.target)?;
        write!(target, "{}", result)?;
        Ok(())
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
