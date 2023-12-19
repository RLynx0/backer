use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub(crate) struct CommandRunError;
impl Error for CommandRunError {}
impl fmt::Display for CommandRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to run command")
    }
}

#[derive(Debug)]
pub(crate) struct SaveLogError;
impl Error for SaveLogError {}
impl fmt::Display for SaveLogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to save Log")
    }
}

#[derive(Debug)]
pub(crate) struct ThreadError;
impl Error for ThreadError {}
impl fmt::Display for ThreadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to run Child Thread")
    }
}
