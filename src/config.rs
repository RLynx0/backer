use std::{collections::HashMap, result, str::FromStr};

use error_stack::{Report, Result, ResultExt};
use serde::Deserialize;

use crate::ctx_string::{Context, CtxString};

use self::error::{
    BackupBuildError, ConfigBuildError, ConfigParseError, LogBuildError, OutLvlParseError,
};

mod error;
mod preview;
mod run;

// --- Constants

const DEFAULT_OUT_SAVE: &str = "output.log";
const DEFAULT_ERR_SAVE: &str = "errors.log";
const DEFAULT_LOG_FORMAT: &str = "${log}";

const SOURCE_BINDING: &str = "source";
const TARGET_BINDING: &str = "target";
const LOG_BINDING: &str = "log";

// --- Merge

trait Merge<T> {
    fn merge(self, fallback: T) -> Self;
}

impl<T> Merge<Option<T>> for Option<T>
where
    T: Merge<T>,
{
    fn merge(self, fallback: Option<T>) -> Self {
        match (self, fallback) {
            (Some(a), Some(b)) => Some(a.merge(b)),
            (a, b) => a.or(b),
        }
    }
}

// --- Deserialized Config

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    template: Option<Template>,
    run: Option<Vec<BackupConfig>>,
    variables: Option<HashMap<String, String>>,
}

impl FromStr for Config {
    type Err = Report<ConfigParseError>;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        toml::from_str(s)
            .change_context(ConfigParseError)
            .attach_printable_lazy(|| format!("{:?} could not be parsed", s))
    }
}

impl Config {
    pub fn build(self) -> Result<(Context, Vec<Backup>), ConfigBuildError> {
        let shared_context = self
            .variables
            .iter()
            .flatten()
            .map(|(key, val)| Ok((key.to_owned(), CtxString::new(val)?)))
            .collect::<Result<Context, _>>()
            .change_context(error::ConfigBuildError)?;

        let runners = self
            .run
            .into_iter()
            .flatten()
            .map(|bcn| bcn.merge(self.template.clone()).build())
            .collect::<Result<Vec<Backup>, _>>()
            .change_context(error::ConfigBuildError)?;

        Ok((shared_context, runners))
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Template {
    output: Option<OutLvlConfig>,
    method: Option<MethodConfig>,
    exclude: Option<Vec<String>>,
    log: Option<LogConfig>,
}

#[derive(Debug, Deserialize)]
struct BackupConfig {
    source: String,
    target: String,
    output: Option<OutLvlConfig>,
    method: Option<MethodConfig>,
    exclude: Option<Vec<String>>,
    log: Option<LogConfig>,
}

impl Merge<Option<Template>> for BackupConfig {
    fn merge(self, fallback: Option<Template>) -> Self {
        match fallback {
            Some(template) => BackupConfig {
                source: self.source,
                target: self.target,
                output: self.output.or(template.output),
                method: self.method.merge(template.method),
                exclude: self.exclude.or(template.exclude),
                log: self.log.merge(template.log),
            },
            None => self,
        }
    }
}

impl BackupConfig {
    fn build(&self) -> Result<Backup, BackupBuildError> {
        Ok(Backup {
            source: CtxString::new(&self.source).change_context(BackupBuildError)?,
            target: CtxString::new(&self.target).change_context(BackupBuildError)?,
            output: match &self.output {
                Some(o) => o.build().change_context(BackupBuildError)?,
                None => OutLvl::default(),
            },
            method: match &self.method {
                Some(m) => m.build(),
                None => Method::default(),
            },
            exclude: match &self.exclude {
                Some(e) => e
                    .iter()
                    .map(|s| CtxString::new(s))
                    .collect::<Result<Vec<_>, _>>()
                    .change_context(BackupBuildError)?,
                None => Vec::with_capacity(0),
            },
            log: match &self.log {
                Some(l) => l.build().change_context(BackupBuildError)?,
                None => Log::default(),
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum OutLvlConfig {
    Explicit(OutLvl),
    Numeric(u8),
}

impl OutLvlConfig {
    fn build(&self) -> Result<OutLvl, OutLvlParseError> {
        match self {
            OutLvlConfig::Explicit(olvl) => Ok(olvl.to_owned()),
            OutLvlConfig::Numeric(0) => Ok(OutLvl::Quiet),
            OutLvlConfig::Numeric(1) => Ok(OutLvl::Default),
            OutLvlConfig::Numeric(2) => Ok(OutLvl::Verbose),
            OutLvlConfig::Numeric(n) => Err(Report::new(OutLvlParseError)
                .attach_printable(format!("{} is not a valid value for output", n))),
        }
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
enum OutLvl {
    #[serde(rename(deserialize = "quiet"))]
    Quiet,

    #[serde(rename(deserialize = "default"))]
    #[default]
    Default,

    #[serde(rename(deserialize = "verbose"))]
    Verbose,
}

#[derive(Clone, Debug, Deserialize)]
struct MethodConfig {
    sudo: Option<bool>,
    delete: Option<bool>,
    dry_run: Option<bool>,
}

impl Merge<MethodConfig> for MethodConfig {
    fn merge(self, fallback: MethodConfig) -> Self {
        MethodConfig {
            sudo: self.sudo.or(fallback.sudo),
            delete: self.delete.or(fallback.delete),
            dry_run: self.dry_run.or(fallback.dry_run),
        }
    }
}

impl MethodConfig {
    fn build(&self) -> Method {
        Method {
            sudo: self.sudo.unwrap_or_default(),
            delete: self.delete.unwrap_or_default(),
            dry_run: self.dry_run.unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct LogConfig {
    append: Option<bool>,
    stderr: Option<String>,
    stdout: Option<String>,
    format: Option<String>,
}

impl Merge<LogConfig> for LogConfig {
    fn merge(self, fallback: LogConfig) -> Self {
        LogConfig {
            append: self.append.or(fallback.append),
            stderr: self.stderr.or(fallback.stderr),
            stdout: self.stdout.or(fallback.stdout),
            format: self.format.or(fallback.format),
        }
    }
}

impl LogConfig {
    fn build(&self) -> Result<Log, LogBuildError> {
        Ok(Log {
            append: self.append.unwrap_or_default(),
            stderr: match &self.stderr {
                Some(s) => CtxString::new(s).change_context(LogBuildError)?,
                None => Log::default().stderr,
            },
            stdout: match &self.stdout {
                Some(s) => CtxString::new(s).change_context(LogBuildError)?,
                None => Log::default().stdout,
            },
            format: match &self.format {
                Some(s) => CtxString::new(s).change_context(LogBuildError)?,
                None => Log::default().format,
            },
        })
    }
}

// --- Finalized Runner

#[derive(Debug)]
pub(crate) struct Backup {
    source: CtxString,
    target: CtxString,
    output: OutLvl,
    method: Method,
    exclude: Vec<CtxString>,
    log: Log,
}

#[derive(Debug, Default)]
struct Method {
    sudo: bool,
    delete: bool,
    dry_run: bool,
}

#[derive(Debug)]
struct Log {
    append: bool,
    stderr: CtxString,
    stdout: CtxString,
    format: CtxString,
}

impl Default for Log {
    fn default() -> Self {
        Log {
            append: false,
            stderr: CtxString::new(DEFAULT_ERR_SAVE).unwrap(),
            stdout: CtxString::new(DEFAULT_OUT_SAVE).unwrap(),
            format: CtxString::new(DEFAULT_LOG_FORMAT).unwrap(),
        }
    }
}
