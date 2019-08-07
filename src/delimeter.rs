use nom::{IResult, named, tag };
use nom::combinator::{ map };
use std::str::from_utf8;

#[derive(Debug, PartialEq, Clone)]
pub enum Delimeter {
    Semicolon,
}

pub fn parse_semicolon(i: &[u8]) -> IResult<&[u8], Delimeter> {
    named!(semi, tag!(";"));
    map(semi, |_| Delimeter::Semicolon)(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Slice;

    #[test]
    fn parses_semicolon() {
        let input = ";";
        let (remaining, delim) = parse_semicolon(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), delim),
            ("", Delimeter::Semicolon))
    }
}