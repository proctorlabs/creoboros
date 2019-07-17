use crate::prelude::*;
use std::time::Duration;

mod executor;
mod timer;

pub trait RunnableAgent {
    fn execute(self) -> Result<()>;
}

macro_rules! impl_agent {
    ($($name:ident, $maker:ident => { $($argname:ident : $argtype:ty),* })+) => {
        #[derive(Debug)]
        pub enum Agent {
            $($name(Box<$name>),)*
        }

        impl Agent {
            $(
                pub fn $maker(name: String, $($argname: $argtype , )*) -> Self {
                    Agent::$name(Box::new($name{
                        name,
                        $($argname , )*
                    }))
                }
            )*
        }

        impl RunnableAgent for Agent {
            fn execute(self) -> Result<()> {
                match self {
                    $(Agent::$name(inner) => inner.execute(),)*
                }
            }
        }

        $(
            impl From<$name> for Agent {
                fn from(agent: $name) -> Self {
                    Agent::$name(Box::new(agent))
                }
            }
        )*

        $(
            #[derive(Debug, Default, Clone)]
            pub struct $name{
                pub name: String,
                $(pub $argname: $argtype , )*
            }

            impl From<Agent> for $name {
                fn from(agent: Agent) -> Self {
                    match agent {
                        Agent::$name(ins) => *ins,
                        _ => $name::default(),
                    }
                }
            }
        )*
    };
}

impl_agent! {
    Executor, executor => { command: String, args: Vec<String> }
    Timer, timer => { interval: Duration }
}
