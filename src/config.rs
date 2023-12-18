use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::{env, fs, path::PathBuf};

use error_stack::{Result, ResultExt};

use self::ctx_string::{CtxParseError, CtxString};

pub use self::{
    processed::{Backup, Context, OutLvl},
    raw::Config,
};

mod ctx_string;
mod processed;
mod raw;

const CONFIG_FILE_NAME: &str = "backer.toml";

#[derive(Debug)]
pub struct ConfigError;
impl Error for ConfigError {}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There was an error parsing the configuration")
    }
}

trait Merge<T> {
    type Output;
    fn merge(self, fallback: T) -> Self::Output;
}
impl<T> Merge<Option<T>> for Option<T>
where
    T: Merge<T, Output = T>,
{
    type Output = Option<T>;
    fn merge(self, fallback: Option<T>) -> Self::Output {
        match (self, fallback) {
            (None, None) => None,
            (None, Some(b)) => Some(b),
            (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(a.merge(b)),
        }
    }
}

pub fn load_config() -> Result<Config, ConfigError> {
    let mut confpath = PathBuf::from(
        env::var("HOME")
            .attach_printable("Could not get Home variable")
            .change_context(ConfigError)?,
    );
    confpath.push(".config");
    confpath.push(CONFIG_FILE_NAME);
    let config = fs::read_to_string(&confpath)
        .change_context(ConfigError)
        .attach_printable(format!("Failed to read {:?}", confpath))?;
    toml::from_str(&config).change_context(ConfigError)
}

pub fn generate_settings(
    Config {
        template,
        run,
        variables,
    }: Config,
) -> Result<Vec<(Context, Backup)>, ConfigError> {
    run.unwrap_or_default()
        .into_iter()
        .map(|backup| backup.merge(template.clone()).merge(Backup::default()))
        .map(|backup_res| {
            let backup = backup_res.change_context(ConfigError)?;
            let variables = variables.clone().unwrap_or_default();

            Ok((
                generate_context(variables, &backup)
                    .change_context(ConfigError)
                    .attach_printable("Failed to generate Backup Context")?,
                backup,
            ))
        })
        .collect::<Result<Vec<_>, _>>()
}

fn generate_context(
    variables: HashMap<String, String>,
    backup: &Backup,
) -> Result<Context, CtxParseError> {
    let mut context = variables
        .into_iter()
        .map(|(k, v)| Ok((k, CtxString::new(&v)?)))
        .collect::<Result<HashMap<_, _>, _>>()?;

    for (key, val) in [("source", &backup.source), ("target", &backup.target)] {
        if !context.contains_key(key) {
            context.insert(key.to_string(), val.clone());
        };
    }

    Ok(context)
}
