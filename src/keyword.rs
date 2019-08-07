use nom::{IResult, named, tag, take_while, ws, take_until1};
use nom::combinator::{ map };
use nom::character::is_alphanumeric;
use crate::delimeter::{Delimeter, parse_semicolon};
use std::fmt;
use std::str::from_utf8;
use crate::atom::{Atom, parse_anything_till_semi, parse_identifier};

const PRAGMA: &str = "pragma";
pub fn parse_pragma_token_keyword(i: &[u8]) -> IResult<&[u8], Atom> {
    map(|b: &[u8]| tag!(b, PRAGMA),
        |b: &[u8]| Atom::Keyword(from_utf8(b).unwrap().to_string()),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::{assert_eq};

    #[test]
    fn parses_pragma_token_keyword() {
        let input = format!("{} solidity ^0.5.6;", PRAGMA);
        assert_eq!(
            parse_pragma_token_keyword(input.as_bytes()).ok().unwrap(),
            (" solidity ^0.5.6;".as_bytes(), Atom::Keyword(PRAGMA.to_string())))
    }
}