use super::*;

use crate::actions::*;
use crate::format::*;
use crate::modules::*;
use std::collections::HashMap;

pub trait Builder<T> {
    fn build(self) -> Result<T>;
}

impl Builder<Vec<Action>> for HashMap<String, ActionConfig> {
    fn build(self) -> Result<Vec<Action>> {
        Ok(self
            .into_iter()
            .map(|(n, c)| match c {
                ActionConfig::Run { script, shell } => {
                    Run::new(n, shell, vec!["-c".into(), script]).into()
                }
                ActionConfig::FileTemplate { template, target } => {
                    FileTemplate::new(n, template, target).into()
                }
            })
            .collect())
    }
}

impl Builder<Formatters> for OutputFormat {
    fn build(self) -> Result<Formatters> {
        Ok(match self {
            OutputFormat::Json => Json.into(),
            OutputFormat::Plain => Plain.into(),
            OutputFormat::Standard => Standard.into(),
        })
    }
}

impl Builder<Vec<Box<dyn DynamicModule>>> for HashMap<String, ModuleConfig> {
    fn build(self) -> Result<Vec<Box<dyn DynamicModule>>> {
        Ok(self
            .into_iter()
            .map(|(n, c)| {
                Ok(match c {
                    ModuleConfig::Exec {
                        command,
                        args,
                        logger,
                        policy,
                    } => Module::<Process>::from(Process::new(
                        n,
                        command,
                        args.get(),
                        logger,
                        policy,
                    ))
                    .into(),
                    ModuleConfig::Script {
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
                    ModuleConfig::Timer {
                        interval,
                        logger,
                        timer,
                    } => Module::<Timer>::from(Timer::new(n, interval, logger, timer.get())).into(),
                    ModuleConfig::FileLogger { path, format, .. } => {
                        Module::<File>::from(File::new(n, path, format.build()?)).into()
                    }
                    ModuleConfig::Console { format, .. } => {
                        Module::<Console>::from(Console::new(n, format.build()?)).into()
                    }
                    ModuleConfig::RegexParser {
                        pattern,
                        forward_to,
                    } => Module::<RegexParser>::from(RegexParser::new(n, pattern, forward_to)?)
                        .into(),
                })
            })
            .collect::<Result<Vec<Box<dyn DynamicModule>>>>()?)
    }
}
