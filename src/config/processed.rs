use std::{collections::HashMap, error::Error, fmt, str::FromStr};

use error_stack::{Report, Result, ResultExt};
use serde::Deserialize;

use super::{ctx_string::CtxString, raw, Merge};

pub type Context = HashMap<String, CtxString>;

#[derive(Debug)]
pub struct ParseConfigError;
impl Error for ParseConfigError {}
impl fmt::Display for ParseConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse Config")
    }
}

#[derive(Debug, Default)]
pub struct Backup {
    pub source: CtxString,
    pub target: CtxString,
    pub output: OutLvl,
    pub method: Method,
    pub exclude: Vec<CtxString>,
    pub log: Log,
}

#[derive(Debug)]
pub struct ParseOutLvlError;
impl Error for ParseOutLvlError {}
impl fmt::Display for ParseOutLvlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse Output Level")
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub enum OutLvl {
    #[serde(rename(deserialize = "quiet"))]
    Quiet,
    #[serde(rename(deserialize = "errors"))]
    Errors,
    #[serde(rename(deserialize = "default"))]
    #[default]
    Default,
    #[serde(rename(deserialize = "verbose"))]
    Verbose,
}
impl TryFrom<raw::OutLvl> for OutLvl {
    type Error = Report<ParseOutLvlError>;
    fn try_from(value: raw::OutLvl) -> std::result::Result<Self, Self::Error> {
        use OutLvl::*;
        match value {
            raw::OutLvl::String(s) => s.parse(),
            raw::OutLvl::Numeric(0) => Ok(Quiet),
            raw::OutLvl::Numeric(1) => Ok(Errors),
            raw::OutLvl::Numeric(2) => Ok(Default),
            raw::OutLvl::Numeric(3) => Ok(Verbose),
            raw::OutLvl::Numeric(x) => Err(Report::new(ParseOutLvlError)
                .attach_printable(format!("{} is an invalid argument for output.", x))),
        }
    }
}
impl FromStr for OutLvl {
    type Err = Report<ParseOutLvlError>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use OutLvl::*;
        match s {
            "quiet" | "q" => Ok(Quiet),
            "errors" | "e" => Ok(Errors),
            "default" | "d" => Ok(Default),
            "verbose" | "v" => Ok(Verbose),
            _ => Err(Report::new(ParseOutLvlError)
                .attach_printable(format!("'{}' is an invalid argument for output.", s))),
        }
    }
}

#[derive(Debug, Default)]
pub struct Method {
    pub sudo: bool,
    pub delete: bool,
    pub dry_run: bool,
}
impl Merge<Method> for raw::Method {
    type Output = Method;
    fn merge(self, fallback: Method) -> Self::Output {
        Method {
            sudo: self.sudo.unwrap_or(fallback.sudo),
            delete: self.delete.unwrap_or(fallback.delete),
            dry_run: self.dry_run.unwrap_or(fallback.dry_run),
        }
    }
}

#[derive(Debug)]
pub struct Log {
    append: bool,
    stderr: CtxString,
    stdout: CtxString,
    format: CtxString,
}
impl Default for Log {
    fn default() -> Self {
        Self {
            append: bool::default(),
            stderr: CtxString::new("errors.log").unwrap(),
            stdout: CtxString::new("output.log").unwrap(),
            format: CtxString::new("${log}").unwrap(),
        }
    }
}
impl Merge<Log> for raw::Log {
    type Output = Log;
    fn merge(self, fallback: Log) -> Self::Output {
        Log {
            append: self.append.unwrap_or(fallback.append),
            stderr: self
                .stderr
                .map(|s| CtxString::new(&s).unwrap())
                .unwrap_or(fallback.stderr),
            stdout: self
                .stdout
                .map(|s| CtxString::new(&s).unwrap())
                .unwrap_or(fallback.stdout),
            format: self
                .format
                .map(|s| CtxString::new(&s).unwrap())
                .unwrap_or(fallback.format),
        }
    }
}

impl Merge<Backup> for raw::Backup {
    type Output = Result<Backup, ParseConfigError>;
    fn merge(self, fallback: Backup) -> Self::Output {
        let exclude = self
            .exclude
            .map(|s| s.into_iter().map(|s| CtxString::new(&s).unwrap()).collect())
            .unwrap_or(fallback.exclude);
        let output = match self.output {
            Some(o) => TryInto::<OutLvl>::try_into(o).change_context(ParseConfigError)?,
            None => fallback.output,
        };
        let method = match self.method {
            Some(m) => m.merge(fallback.method),
            None => fallback.method,
        };
        let log = match self.log {
            Some(l) => l.merge(fallback.log),
            None => fallback.log,
        };

        Ok(Backup {
            source: CtxString::new(&self.source)
                .change_context(ParseConfigError)
                .attach_printable(format!("Failed to parse {:?}", self.source))?,
            target: CtxString::new(&self.target)
                .change_context(ParseConfigError)
                .attach_printable(format!("Failed to parse {:?}", self.target))?,
            output,
            method,
            exclude,
            log,
        })
    }
}
