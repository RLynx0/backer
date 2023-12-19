use std::{error::Error, fmt};

#[derive(Debug)]
pub struct CtxWriteError;

impl Error for CtxWriteError {}

impl fmt::Display for CtxWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to write Context String")
    }
}

#[derive(Debug)]
pub struct CtxParseError;

impl Error for CtxParseError {}

impl fmt::Display for CtxParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse Context String")
    }
}
