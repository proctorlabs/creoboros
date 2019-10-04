use crate::prelude::*;
pub use ser::{ConfigExpression, ConfigTemplate};
use templar::{Context, SharedContext, Templar, TemplateTree};

mod ser;

lazy_static! {
    pub static ref TEMPLAR: Templar = Templar::default();
    pub static ref CONTEXT: SharedContext = SharedContext::default();
}

pub fn context_set_value(doc: &templar::Document) -> Result<()> {
    let t: TemplateTree = TEMPLAR.parse(doc)?;
    CONTEXT.set(t)?;
    Ok(())
}
