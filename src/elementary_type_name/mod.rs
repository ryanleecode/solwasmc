use crate::elementary_type_name::uint::parse as parse_uInt;
pub use crate::elementary_type_name::uint::UInt;
use nom::{branch::alt, combinator::map, named, tag, IResult};

mod uint;

const ADDRESS: &str = r#"address"#;
const BOOL: &str = r#"bool"#;
const STRING: &str = r#"string"#;
const INT: &str = r#"int"#;
const BYTE: &str = r#"byte"#;
const FIXED: &str = r#"fixed"#;
const UFIXED: &str = r#"ufixed"#;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ElementaryTypeName {
    Address,
    Bool,
    String,
    Int,
    UInt(UInt),
    Byte,
    Fixed,
    Ufixed,
}

fn parse_address(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
    named!(semi, tag!(ADDRESS));
    map(semi, |_| ElementaryTypeName::Address)(i)
}
fn parse_bool(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
    named!(semi, tag!(BOOL));
    map(semi, |_| ElementaryTypeName::Bool)(i)
}
fn parse_string(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
    named!(semi, tag!(STRING));
    map(semi, |_| ElementaryTypeName::String)(i)
}
fn parse_int(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
    named!(semi, tag!(INT));
    map(semi, |_| ElementaryTypeName::Int)(i)
}
fn parse_byte(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
    named!(semi, tag!(BYTE));
    map(semi, |_| ElementaryTypeName::Byte)(i)
}
fn parse_fixed(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
    named!(semi, tag!(FIXED));
    map(semi, |_| ElementaryTypeName::Fixed)(i)
}
fn parse_ufixed(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
    named!(semi, tag!(UFIXED));
    map(semi, |_| ElementaryTypeName::Ufixed)(i)
}
pub fn parse(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
    alt((
        parse_address,
        parse_bool,
        parse_string,
        parse_int,
        map(parse_uInt, |x| ElementaryTypeName::UInt(x)),
        parse_byte,
        parse_fixed,
        parse_ufixed,
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::from_utf8;

    #[test]
    fn parses_address() {
        let input = format!("{} a", ADDRESS);
        let (remaining, name) = parse_address(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), name),
            (" a", ElementaryTypeName::Address)
        )
    }
    #[test]
    fn parses_bool() {
        let input = format!("{} a", BOOL);
        let (remaining, name) = parse_bool(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), name),
            (" a", ElementaryTypeName::Bool)
        )
    }
    #[test]
    fn parses_string() {
        let input = format!("{} a", STRING);
        let (remaining, name) = parse_string(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), name),
            (" a", ElementaryTypeName::String)
        )
    }
    #[test]
    fn parses_int() {
        let input = format!("{} a", INT);
        let (remaining, name) = parse_int(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), name),
            (" a", ElementaryTypeName::Int)
        )
    }

    #[test]
    fn parses_byte() {
        let input = format!("{} a", BYTE);
        let (remaining, name) = parse_byte(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), name),
            (" a", ElementaryTypeName::Byte)
        )
    }
    #[test]
    fn parses_fixed() {
        let input = format!("{} a", FIXED);
        let (remaining, name) = parse_fixed(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), name),
            (" a", ElementaryTypeName::Fixed)
        )
    }
    #[test]
    fn parses_ufixed() {
        let input = format!("{} a", UFIXED);
        let (remaining, name) = parse_ufixed(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), name),
            (" a", ElementaryTypeName::Ufixed)
        )
    }
}
