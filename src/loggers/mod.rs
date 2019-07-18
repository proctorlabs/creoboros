use crate::prelude::*;
use std::path::PathBuf;
use tokio::sync::mpsc::*;
use crate::runtime::{Message};

mod file;
mod stdout;

pub trait LoggerSink {
    fn log(self) -> Result<()>;
}

macro_rules! impl_logger {
    ($($name:ident, $maker:ident => { $($argname:ident : $argtype:ty),* })+) => {
        #[derive(Debug)]
        pub enum Logger {
            $($name(Box<$name>),)*
        }

        impl Logger {
            pub fn get_name(&self) -> String {
                match self {
                    $(Logger::$name(inner) => inner.name.to_string(),)*
                }
            }

            $(
                pub fn $maker(name: String, $($argname: $argtype , )*) -> Self {
                            let (sender, receiver) = unbounded_channel();
                    Logger::$name(Box::new($name{
                        name,
                        receiver,
                        sender,
                        $($argname , )*
                    }))
                }
            )*
        }

        impl LoggerSink for Logger {
            fn log(self) -> Result<()> {
                match self {
                    $(Logger::$name(inner) => inner.log(),)*
                }
            }
        }

        $(
            impl From<$name> for Logger {
                fn from(logger: $name) -> Self {
                    Logger::$name(Box::new(logger))
                }
            }
        )*

        $(
            #[derive(Debug)]
            pub struct $name{
                pub name: String,
                pub sender: UnboundedSender<Message>,
                pub receiver: UnboundedReceiver<Message>,
                $(pub $argname: $argtype , )*
            }

            impl From<Logger> for $name {
                fn from(logger: Logger) -> Self {
                    match logger {
                        Logger::$name(ins) => *ins,
                        _ => panic!("Invalid logger type"),
                    }
                }
            }
        )*
    };
}

impl_logger! {
    Stdout, stdout => { }
    File, file => { path: PathBuf }
}
