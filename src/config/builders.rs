use super::*;

use crate::actions::*;
use crate::modules::*;
use std::collections::HashMap;

pub trait Builder<T> {
    fn build(self) -> T;
}

impl Builder<Vec<Action>> for HashMap<String, ActionConfig> {
    fn build(self) -> Vec<Action> {
        self.into_iter()
            .map(|(n, c)| match c {
                ActionConfig::Run { script, shell } => {
                    Run::new(n, shell, vec!["-c".into(), script]).into()
                }
            })
            .collect()
    }
}

impl Builder<Vec<Box<dyn DynamicModule>>> for HashMap<String, LoggerConfig> {
    fn build(self) -> Vec<Box<dyn DynamicModule>> {
        self.into_iter()
            .map(|(n, c)| match c {
                LoggerConfig::Stdout => Module::<Stdout>::from(Stdout::new(n)).into(),
                LoggerConfig::File { path } => Module::<File>::from(File::new(n, path)).into(),
            })
            .collect()
    }
}

impl Builder<Vec<Box<dyn DynamicModule>>> for HashMap<String, AgentConfig> {
    fn build(self) -> Vec<Box<dyn DynamicModule>> {
        self.into_iter()
            .map(|(n, c)| match c {
                AgentConfig::Exec {
                    command,
                    args,
                    logger,
                    policy,
                } => Module::<Process>::from(Process::new(n, command, args.get(), logger, policy))
                    .into(),
                AgentConfig::Script {
                    shell,
                    script,
                    logger,
                    policy,
                } => Module::<Process>::from(Process::new(
                    n,
                    shell,
                    vec!["-c".into(), script],
                    logger,
                    policy,
                ))
                .into(),
                AgentConfig::Timer { interval, logger } => {
                    Module::<Timer>::from(Timer::new(n, interval, logger)).into()
                }
            })
            .collect()
    }
}
