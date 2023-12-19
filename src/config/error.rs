use std::fmt;

use std::error::Error;

#[derive(Debug)]
pub(crate) struct ConfigParseError;

impl Error for ConfigParseError {}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse Config")
    }
}

#[derive(Debug)]
pub(crate) struct ConfigBuildError;

impl Error for ConfigBuildError {}

impl fmt::Display for ConfigBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to build Runners from Config")
    }
}

#[derive(Debug)]
pub(crate) struct BackupBuildError;

impl Error for BackupBuildError {}

impl fmt::Display for BackupBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to build Backup Config")
    }
}

#[derive(Debug)]
pub(crate) struct BackupCompileError;

impl Error for BackupCompileError {}

impl fmt::Display for BackupCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to compile Backup")
    }
}

#[derive(Debug)]
pub(crate) struct BackupRunError;

impl Error for BackupRunError {}

impl fmt::Display for BackupRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to run Backup")
    }
}

#[derive(Debug)]
pub(crate) struct OutLvlParseError;

impl Error for OutLvlParseError {}

impl fmt::Display for OutLvlParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse Output Level")
    }
}

#[derive(Debug)]
pub(crate) struct LogBuildError;

impl Error for LogBuildError {}

impl fmt::Display for LogBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to build Log Config")
    }
}
