use std::collections::HashMap;

use chrono::Local;
use error_stack::{Report, Result};

use self::{
    error::{CtxParseError, CtxWriteError},
    parser::ctx_str as parse_ctx_str,
};

mod error;
mod parser;

pub type Context = HashMap<String, CtxString>;

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

    pub fn literal(string: &str) -> CtxString {
        CtxString(vec![Token::Literal(string.to_owned())])
    }

    pub fn evaluate(&self, context: &Context) -> Result<String, CtxWriteError> {
        self.0
            .iter()
            .map(|token| match token {
                Token::Literal(s) => Ok(s.clone()),
                Token::Var(v) => context
                    .get(v)
                    .map(|s| s.evaluate(context))
                    .unwrap_or(Err(Report::new(CtxWriteError)
                        .attach_printable(format!("Variable {:?} is not defined", v)))),
                Token::DateTime(d) => Ok(Local::now().format(d).to_string()),
            })
            .collect::<Result<String, _>>()
    }
}

#[cfg(test)]
mod tests;
