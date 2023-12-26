use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub(crate) struct ReadConfigError;
impl Error for ReadConfigError {}
impl Display for ReadConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to read configuration")
    }
}

#[derive(Debug)]
pub struct SaveLogError;
impl Error for SaveLogError {}
impl Display for SaveLogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to save log")
    }
}

#[derive(Debug)]
pub(crate) struct CheckError;
impl Error for CheckError {}
impl Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File check failed")
    }
}
