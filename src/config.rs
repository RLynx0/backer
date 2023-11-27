use std::error::Error;
use std::fmt;
use std::{env, fs, path::PathBuf};

use error_stack::{Result, ResultExt};

pub use self::{
    processed::{Backup, OutLvl},
    raw::Config,
};

mod ctx_string;
mod processed;
mod raw;

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
    let mut confpath = PathBuf::from(env::var("HOME").expect("Could not get Home variable"));
    confpath.push(".config");
    confpath.push("backer.toml");
    let config = fs::read_to_string(confpath).expect("Could not read config path");
    toml::from_str(&config).change_context(ConfigError)
}

pub fn generate_settings(config: raw::Config) -> Result<Vec<Backup>, ConfigError> {
    config
        .run
        .unwrap_or_default()
        .into_iter()
        .map(|backup| {
            backup
                .merge(config.template.clone())
                .merge(Backup::default())
        })
        .collect::<Result<Vec<_>, _>>()
        .change_context(ConfigError)
}
