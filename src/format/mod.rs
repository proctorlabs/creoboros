use super::*;

mod formatters;

pub use formatters::*;

pub trait Formatter {
    fn format(&self, log: Document) -> String;
}

#[derive(Debug, From, Clone)]
pub enum Formatters {
    Plain(Plain),
    Standard(Standard),
    Json(Json),
}

impl Default for Formatters {
    fn default() -> Self {
        Standard.into()
    }
}

impl Formatter for Formatters {
    fn format(&self, log: Document) -> String {
        match self {
            Formatters::Plain(c) => c.format(log),
            Formatters::Standard(c) => c.format(log),
            Formatters::Json(c) => c.format(log),
        }
    }
}
