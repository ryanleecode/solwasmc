use crate::atom::{
    delimiter::{parse_semicolon, Delimiter},
    keyword::parse_pragma,
    parse_anything_till_semi, parse_identifier, Atom,
};
use nom::combinator::map;
use nom::{named, ws, IResult};

#[derive(Debug, PartialEq, Clone)]
pub struct PragmaDirective {
    pub name: String,
    pub value: String,
}

pub fn parse_pragma_directive(i: &[u8]) -> IResult<&[u8], PragmaDirective> {
    named!(tuple<&[u8], (Atom, String, Atom, Delimiter)>, ws!(tuple!(
        parse_pragma, parse_identifier, parse_anything_till_semi, parse_semicolon)));
    map(tuple, |t| {
        let (_, identifier, version, _) = t;
        PragmaDirective {
            name: identifier,
            value: version.to_string(),
        }
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use std::str::from_utf8;

    #[test]
    fn parses_pragma_directive() {
        let input = "pragma solidity ^0.5.6;";
        let (remaining, directive) = parse_pragma_directive(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), directive),
            (
                "",
                PragmaDirective {
                    name: "solidity".to_string(),
                    value: "^0.5.6".to_string()
                }
            )
        )
    }
}
