use nom::{IResult, named, tag, take_while};
use nom::combinator::{ map };
use nom::character::is_alphanumeric;
use nom::error::{VerboseError, context};
use std::str::from_utf8;
/*
#[derive(Debug)]
pub struct PragmaDirective {
    pub name: str,
    pub value: str,
    pub range: [u128; 2]
}*/

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Keyword(String),
    Identifier(String)
}

fn parse_pragma_token(i: &[u8]) -> IResult<&[u8], Atom> {
    map(|j| tag!(j, "pragma"),
        |s| Atom::Keyword(from_utf8(s).unwrap().to_string()))(i)
}

fn parse_identifier<'a>(i: &[u8]) -> IResult<&[u8], Atom> {
    named!(alphanum, take_while!(is_alphanumeric));
    map(|j|alphanum(j),
        |s| Atom::Identifier(from_utf8(s).unwrap().to_string()))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::{assert_eq};

    #[test]
    fn parses_pragma_token() {
        let input = "pragma solidity ^0.5.6;";
        assert_eq!(
            parse_pragma_token(input.as_bytes()).ok(),
            Some((" solidity ^0.5.6;".as_bytes(), Atom::Keyword("pragma".to_string()))))
    }

    #[test]
    fn parses_identifier() {
        let input = "solidity ^0.5.6;";
        assert_eq!(
            parse_identifier(input.as_bytes()).ok(),
            Some((" ^0.5.6;".as_bytes(), Atom::Identifier("solidity".to_string()))))
    }
}