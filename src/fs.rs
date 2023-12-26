use std::{
    env,
    fs::{read_to_string, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use error_stack::{Result, ResultExt};

const HOME_VARIABLE: &str = "HOME";
const CONFIG_DIR: &str = ".config";
const CONFIG_FILE_NAME: &str = "backer.toml";

mod error;

pub use error::SaveLogError;

pub(crate) fn read_config() -> Result<String, error::ReadConfigError> {
    let home = env::var(HOME_VARIABLE).change_context(error::ReadConfigError)?;
    let mut confpath = PathBuf::from(home);
    confpath.push(CONFIG_DIR);
    confpath.push(CONFIG_FILE_NAME);
    Ok(read_to_string(&confpath).change_context(error::ReadConfigError)?)
}

pub(crate) fn save(content: &str, path: &Path, append: bool) -> Result<(), error::SaveLogError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .open(path)
        .change_context(error::SaveLogError)
        .attach_printable_lazy(|| format!("Failed to open {:?}", path))?;

    writeln!(file, "{}", content)
        .change_context(error::SaveLogError)
        .attach_printable_lazy(|| format!("Failed to write to {:?}", file))
}
