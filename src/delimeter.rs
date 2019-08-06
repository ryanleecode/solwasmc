use nom::{IResult, named, is_a };
use nom::combinator::{ map };

#[derive(Debug, PartialEq, Clone)]
pub enum Delimeter {
    Semicolon,
}

fn parse_semicolon(i: &[u8]) -> IResult<&[u8], Delimeter> {
    named!(semi, is_a!(";"));
    map(semi, |s| Delimeter::Semicolon)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_semicolon() {
        let input = ";\ncontract Test{}";
        assert_eq!(
            parse_semicolon(input.as_bytes()).ok(),
            Some(("\ncontract Test{}".as_bytes(), Delimeter::Semicolon)))
    }
}