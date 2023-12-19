use std::{collections::HashMap, error::Error, fmt::Display};

use chrono::Local;
use error_stack::{Report, Result};

use self::parser::ctx_str as parse_ctx_str;

mod parser;

pub type Context = HashMap<String, CtxString>;

#[derive(Debug)]
pub struct CtxWriteError;
impl Error for CtxWriteError {}
impl Display for CtxWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to write Context String")
    }
}
#[derive(Debug)]
pub struct CtxParseError;
impl Error for CtxParseError {}
impl Display for CtxParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse Context String")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Literal(String),
    Var(String),
    DateTime(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CtxString(Vec<Token>);

impl CtxString {
    pub fn new(string: &str) -> Result<CtxString, CtxParseError> {
        let (i, s) = parse_ctx_str(string).or(Err(CtxParseError))?;
        if !i.is_empty() {
            Err(Report::new(CtxParseError)
                .attach_printable(format!("Couldn't parse complete input: {:?}", i)))
        } else {
            Ok(s)
        }
    }

    pub fn to_string(&self, context: &Context) -> Result<String, CtxWriteError> {
        self.0
            .iter()
            .map(|token| match token {
                Token::Literal(s) => Ok(s.clone()),
                Token::Var(v) => context
                    .get(v)
                    .map(|s| s.to_string(context))
                    .unwrap_or(Err(Report::new(CtxWriteError)
                        .attach_printable(format!("Variable {} is not defined", v)))),
                Token::DateTime(d) => Ok(Local::now().format(d).to_string()),
            })
            .collect::<Result<String, _>>()
    }
}

#[cfg(test)]
mod tests;
