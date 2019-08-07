use nom::{IResult, named, tag, take_while, ws, take_until1};
use nom::combinator::{ map };
use nom::character::is_alphanumeric;
use crate::delimeter::{Delimeter, parse_semicolon};
use std::fmt;
use std::str::from_utf8;

#[derive(Debug, PartialEq, Clone)]
pub struct PragmaDirective {
    pub name:  String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Keyword(String),
    Identifier(String),
    Anything(String)
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut word = "";
        match self {
            Atom::Keyword(w) => word = w,
            Atom::Identifier(w) => word = w,
            Atom::Anything(w) => word = w,
            _ => {}
        }
        write!(f, "{}", word)
    }
}

fn parse_pragma_token(i: &[u8]) -> IResult<&[u8], Atom> {
    map(|b: &[u8]| tag!(b, "pragma"),
        |b: &[u8]| Atom::Keyword(from_utf8(b).unwrap().to_string()),
    )(i)
}

fn parse_identifier<'a>(i: &[u8]) -> IResult<&[u8], Atom> {
    named!(alphanum, take_while!(is_alphanumeric));
    map(|b: &[u8]|alphanum(b),
        |b: &[u8]| Atom::Identifier(from_utf8(b).unwrap().to_string()),
    )(i)
}

fn parse_anything_till_semi(i: &[u8]) -> IResult<&[u8], Atom> {
    named!(untilsemi, take_until1!(";"));
    map(|b: &[u8]|untilsemi(b),
        |b: &[u8]| Atom::Anything(from_utf8(b).unwrap().to_string()),
    )(i)
}

fn parse_pragma_statement(i: &[u8]) -> IResult<&[u8], Box<PragmaDirective>> {
    named!(tuple<&[u8], (Atom, Atom, Atom, Delimeter)>, ws!(tuple!(
        parse_pragma_token, parse_identifier, parse_anything_till_semi, parse_semicolon)));
    map(tuple, |t| {
        let (_, identifier, version, _) = t;
        Box::new(PragmaDirective{name: identifier.to_string(), value: version.to_string()})
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::{assert_eq};

    #[test]
    fn parses_pragma_token() {
        let input = "pragma solidity ^0.5.6;";
        assert_eq!(
            parse_pragma_token(input.as_bytes()).ok().unwrap(),
            (" solidity ^0.5.6;".as_bytes(), Atom::Keyword("pragma".to_string())))
    }

    #[test]
    fn parses_identifier() {
        let input = "solidity ^0.5.6;";
        let (remaining, atom) = parse_identifier(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), atom),
            (" ^0.5.6;", Atom::Identifier("solidity".to_string())))
    }

    #[test]
    fn parses_anything_till_semi() {
        let input = "^0.5.6!@#$%^&*()_+=;";
        let (remaining, atom) = parse_anything_till_semi(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), atom),
            (";", Atom::Anything("^0.5.6!@#$%^&*()_+=".to_string())))
    }

    #[test]
    fn parses_pragma_statement() {
        let input = "pragma solidity ^0.5.6;";
        let (remaining, directive) = parse_pragma_statement(
            input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), directive.as_ref()),
            ("", &PragmaDirective{ name: "solidity".to_string(), value: "^0.5.6".to_string() }))
    }
}