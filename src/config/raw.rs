use std::collections::HashMap;

use serde::Deserialize;

use super::Merge;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub template: Option<Template>,
    pub run: Option<Vec<Backup>>,
    pub variables: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Template {
    output: Option<OutLvl>,
    method: Option<Method>,
    exclude: Option<Vec<String>>,
    log: Option<Log>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum OutLvl {
    String(String),
    Numeric(u8),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Backup {
    pub source: String,
    pub target: String,
    pub output: Option<OutLvl>,
    pub method: Option<Method>,
    pub exclude: Option<Vec<String>>,
    pub log: Option<Log>,
}
impl Merge<Option<Template>> for Backup {
    type Output = Backup;
    fn merge(self, fallback: Option<Template>) -> Self::Output {
        match fallback {
            None => self,
            Some(template) => Backup {
                source: self.source,
                target: self.target,
                output: self.output.or(template.output),
                method: self.method.merge(template.method),
                exclude: self.exclude.or(template.exclude),
                log: self.log.merge(template.log),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Method {
    pub sudo: Option<bool>,
    pub delete: Option<bool>,
    pub dry_run: Option<bool>,
}
impl Merge<Method> for Method {
    type Output = Method;
    fn merge(self, fallback: Method) -> Self::Output {
        Method {
            sudo: self.sudo.or(fallback.sudo),
            delete: self.delete.or(fallback.delete),
            dry_run: self.dry_run.or(fallback.dry_run),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Log {
    pub append: Option<bool>,
    pub stderr: Option<String>,
    pub stdout: Option<String>,
    pub format: Option<String>,
}
impl Merge<Log> for Log {
    type Output = Log;
    fn merge(self, fallback: Log) -> Self::Output {
        Log {
            append: self.append.or(fallback.append),
            stderr: self.stderr.or(fallback.stderr),
            stdout: self.stdout.or(fallback.stdout),
            format: self.format.or(fallback.format),
        }
    }
}
