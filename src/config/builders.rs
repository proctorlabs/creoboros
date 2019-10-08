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
        let result = self
            .into_iter()
            .map(|(n, c)| match c {
                ActionConfig::Run { script, shell } => {
                    Run::new(n, shell, vec!["-c".into(), script]).into()
                }
                ActionConfig::FileTemplate { template, target } => {
                    FileTemplate::new(n, template, target).into()
                }
                ActionConfig::Action { action } => ActionAction::new(n, action.get()).into(),
            })
            .collect::<Vec<Action>>();
        Ok(result)
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

const NGINX: &str = r#"(?P<remote_ip>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}) - - \[(?P<nginx_timestamp>\d{2}/[a-zA-Z]{3}/\d{4}:\d{2}:\d{2}:\d{2} (\+|\-)\d{4})\] (("(GET|POST) )(?P<url>.+) (HTTP/1\.1")) (?P<status_code>\d{3}) (?P<bytes_sent>\d+) (["](?P<referer>(\-)|(.+))["]) (["](?P<useragent>.+)["])"#;

impl Builder<String> for ParserFormat {
    fn build(self) -> Result<String> {
        match self {
            Self::Pattern { pattern } => Ok(pattern),
            Self::BuiltIn {
                built_in_pattern: BuiltInParserFormat::Nginx,
            } => Ok(NGINX.into()),
        }
    }
}

impl Builder<Vec<Box<dyn DynamicModule>>> for HashMap<String, ModuleConfig> {
    fn build(self) -> Result<Vec<Box<dyn DynamicModule>>> {
        let mut result = self
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
                    } => Module::<RegexParser>::from(RegexParser::new(
                        n,
                        pattern.build()?,
                        forward_to,
                    )?)
                    .into(),
                    ModuleConfig::Start { start, logger } => {
                        Module::<Start>::from(Start::new(n, logger, start.get())).into()
                    }
                })
            })
            .collect::<Result<Vec<Box<dyn DynamicModule>>>>()?;

        result.sort_unstable_by_key(|k| k.priority());
        Ok(result)
    }
}
