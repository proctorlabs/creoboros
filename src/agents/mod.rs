use std::time::Duration;

macro_rules! impl_agent {
    ($($name:ident, $maker:ident => { $($argname:ident : $argtype:ty),* })+) => {
        #[derive(Debug)]
        pub enum Agent {
            $($name($name),)*
        }

        impl Agent {
            $(
                pub fn $maker($($argname: $argtype , )*) -> Self {
                    Agent::$name($name{
                        $($argname , )*
                    })
                }
            )*
        }

        $(
            impl From<$name> for Agent {
                fn from(agent: $name) -> Self {
                    Agent::$name(agent)
                }
            }
        )*

        $(
            #[derive(Debug, Default)]
            pub struct $name{
                $(pub $argname: $argtype , )*
            }

            impl From<Agent> for $name {
                fn from(agent: Agent) -> Self {
                    match agent {
                        Agent::$name(ins) => ins,
                        _ => $name::default(),
                    }
                }
            }
        )*
    };
}

impl_agent! {
    Executor, executor => { path: String }
    Timer, _timer => { duration: Duration }
}
