use nom::{IResult, named, tag, take_while, ws, take_until1};
use nom::combinator::{ map };
use nom::character::is_alphanumeric;
use crate::delimeter::{Delimeter, parse_semicolon};
use std::fmt;
use std::str::from_utf8;
use crate::atom::{Atom, parse_anything_till_semi, parse_identifier};
use crate::keyword::{parse_pragma_token_keyword};

#[derive(Debug, PartialEq, Clone)]
pub struct PragmaDirective {
    pub name:  String,
    pub value: String,
}

fn parse_pragma_statement(i: &[u8]) -> IResult<&[u8], Box<PragmaDirective>> {
    named!(tuple<&[u8], (Atom, Atom, Atom, Delimeter)>, ws!(tuple!(
        parse_pragma_token_keyword, parse_identifier, parse_anything_till_semi, parse_semicolon)));
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
    fn parses_pragma_statement() {
        let input = "pragma solidity ^0.5.6;";
        let (remaining, directive) = parse_pragma_statement(
            input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), directive.as_ref()),
            ("", &PragmaDirective{ name: "solidity".to_string(), value: "^0.5.6".to_string() }))
    }
}