use std::{collections::HashMap, fmt::Display};

type Printable<'a> = &'a dyn Fn() -> &'a dyn Display;

pub enum Token {
    Literal(String),
    Var(String),
}

pub struct CtxString<'a> {
    context: HashMap<String, Printable<'a>>,
    string: Vec<Token>,
}

impl<'a> CtxString<'a> {
    pub fn new(string: &str, context: HashMap<String, Printable<'a>>) -> CtxString<'a> {
        todo!()
    }
    pub fn write_var(&'a mut self, varname: &str, printable: Printable<'a>) {
        self.context.insert(varname.to_owned(), printable);
    }
}

impl<'a> Display for CtxString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .string
            .iter()
            .map(|t| match t {
                Token::Literal(s) => s.to_owned(),
                Token::Var(v) => (self.context.get(v).unwrap())().to_string(),
            })
            .collect::<String>();

        write!(f, "{}", string)
    }
}
