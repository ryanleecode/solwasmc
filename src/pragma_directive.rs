use crate::token::Token;
use nom::{IResult, alt, is_a , named, tag};
use nom::combinator::{ map };
use nom::error::{VerboseError, context};
/*
#[derive(Debug)]
pub struct PragmaDirective {
    pub name: str,
    pub value: str,
    pub range: [u128; 2]
}*/

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Keyword(String)
}

fn parse_pragma_token(i: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    map(context("keyword", |s: &str|tag!(s, "pragma")), |pragma: &str| {
        Atom::Keyword(pragma.to_string())
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pragma_token() {
        let input = "pragma solidity ^0.5.6;";
        assert_eq!(
            parse_pragma_token(input).ok(),
            Some((" solidity ^0.5.6;", Atom::Keyword("pragma".to_string()))))
    }
}