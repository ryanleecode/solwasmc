use crate::atom::{
    delimiter::Delimiter, elementary_type_name::ElementaryTypeName,
    storage_location::StorageLocation,
};
use nom::{
    character::{
        complete::{char, multispace0, multispace1},
        is_alphanumeric,
    },
    combinator::map,
    multi::separated_nonempty_list,
    named,
    sequence::{delimited, preceded},
    take_until1, take_while, IResult,
};
use std::fmt;
use std::str::from_utf8;

#[allow(dead_code)]
pub mod delimiter;
#[allow(dead_code)]
pub mod elementary_type_name;
#[allow(dead_code)]
pub mod keyword;
#[allow(dead_code)]
pub mod reserved;
#[allow(dead_code)]
pub mod storage_location;

pub type Identifier = String;

#[derive(Debug, PartialEq, Clone)]
pub enum TypeName {
    ElementaryTypeName(ElementaryTypeName),
    UserDefinedTypeName,
    // TODO: Mapping
    // TODO: ArrayTypeName
    // TODO: FunctionTypeName
    // TODO: ( 'address' 'payable' )
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Reserved(String),
    Keyword(String),
    Identifier(Identifier),
    Anything(String),
    Delimiter(Delimiter),
    TypeName(TypeName),
    StorageLocation(StorageLocation),
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

pub fn parse_user_defined_type_name(i: &[u8]) -> IResult<&[u8], TypeName> {
    map(separated_nonempty_list(char('.'), parse_identifier), |_| {
        TypeName::UserDefinedTypeName
    })(i)
}

pub fn parse_identifier<'a>(i: &[u8]) -> IResult<&[u8], Atom> {
    named!(alphanum, take_while!(is_alphanumeric));
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
    fn parses_anything_till_semi() {
        let input = "^0.5.6!@#$%^&*()_+=;";
        let (remaining, atom) = parse_anything_till_semi(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), atom),
            (";", Atom::Anything("^0.5.6!@#$%^&*()_+=".to_string()))
        )
    }

    #[test]
    fn parses_user_defined_type_name() {
        let input = "OpenZepp.ERC20.ABC {";
        let (remaining, typename) = parse_user_defined_type_name(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), typename),
            (" {", TypeName::UserDefinedTypeName)
        )
    }
}
