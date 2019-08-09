use crate::atom::delimiter::Delimiter;
use nom::{character::is_alphanumeric, combinator::map, named, take_until1, take_while1, IResult};
use std::fmt;
use std::str::from_utf8;

#[allow(dead_code)]
pub mod delimiter;
#[allow(dead_code)]
pub mod keyword;
#[allow(dead_code)]
pub mod reserved;

pub type Identifier = String;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Reserved(String),
    Keyword(String),
    Identifier(Identifier),
    Anything(String),
    Delimiter(Delimiter),
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

pub fn parse_identifier<'a>(i: &[u8]) -> IResult<&[u8], Atom> {
    named!(alphanum, take_while1!(is_alphanumeric));
    map(
        |b: &[u8]| alphanum(b),
        |b: &[u8]| Atom::Identifier(from_utf8(b).unwrap().to_string()),
    )(i)
}

pub fn parse_anything_till_semi(i: &[u8]) -> IResult<&[u8], Atom> {
    named!(untilsemi, take_until1!(";"));
    map(
        |b: &[u8]| untilsemi(b),
        |b: &[u8]| Atom::Anything(from_utf8(b).unwrap().to_string()),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn parses_identifier() {
        let input = "solidity ^0.5.6;";
        let (remaining, atom) = parse_identifier(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), atom),
            (" ^0.5.6;", Atom::Identifier("solidity".to_string()))
        )
    }

    #[test]
    fn parse_identifier_should_never_be_an_empty_str() {
        let input = "           ";
        let result = parse_identifier(input.as_bytes());
        result.expect_err("should be None");
    }

    #[test]
    fn parses_anything_till_semi() {
        let input = "^0.5.6!@#$%^&*()_+=;";
        let (remaining, atom) = parse_anything_till_semi(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), atom),
            (";", Atom::Anything("^0.5.6!@#$%^&*()_+=".to_string()))
        )
    }
}
