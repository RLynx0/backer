use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take},
    combinator::opt,
    multi::{many0, many1},
    sequence::{delimited, tuple},
    IResult,
};

use super::{CtxString, Token};

pub fn ctx_str(input: &str) -> IResult<&str, CtxString> {
    let (input, tokens) = many0(alt((date, var, literal)))(input)?;
    Ok((input, CtxString(tokens)))
}

fn literal(input: &str) -> IResult<&str, Token> {
    let (input, parts) = many1(alt((is_not("\\%$"), escape)))(input)?;
    Ok((input, Token::Literal(parts.concat())))
}

fn date(input: &str) -> IResult<&str, Token> {
    let (input, (_, syms, char)) = tuple((tag("%"), opt(is_a("_:")), take(1usize)))(input)?;
    let fmt = format!("%{}{}", syms.unwrap_or_default(), char);
    Ok((input, Token::DateTime(fmt)))
}

fn var(input: &str) -> IResult<&str, Token> {
    let (input, parts) = delimited(tag("${"), var_str, tag("}"))(input)?;
    Ok((input, Token::Var(parts)))
}

fn var_str(input: &str) -> IResult<&str, String> {
    let (input, res) = many1(alt((is_not("\\}"), escape)))(input)?;
    Ok((input, res.concat()))
}

fn escape(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("\\")(input)?;
    take(1usize)(input)
}
