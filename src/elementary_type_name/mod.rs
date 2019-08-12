// GENERATED: DO NOT EDIT
use nom::{
  named,
  tag,
  IResult,
  combinator::{map},
  branch::{alt}
};
pub use crate::elementary_type_name::uint::{UInt};
use crate::elementary_type_name::uint::{parse as parse_uInt};

mod uint;

const ADDRESS: &str = r#"address"#;
const BOOL: &str = r#"bool"#;
const STRING: &str = r#"string"#;
const INT: &str = r#"Int"#;
const UINT: &str = r#"Uint"#;
const BYTE: &str = r#"Byte"#;
const FIXED: &str = r#"Fixed"#;
const UFIXED: &str = r#"Ufixed"#;

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
  named!(semi, tag!(r#"address"#));
  map(semi, |_| ElementaryTypeName::Address)(i)
}
fn parse_bool(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
  named!(semi, tag!(r#"bool"#));
  map(semi, |_| ElementaryTypeName::Bool)(i)
}
fn parse_string(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
  named!(semi, tag!(r#"string"#));
  map(semi, |_| ElementaryTypeName::String)(i)
}
fn parse_int(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
  named!(semi, tag!(r#"Int"#));
  map(semi, |_| ElementaryTypeName::Int)(i)
}

fn parse_byte(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
  named!(semi, tag!(r#"Byte"#));
  map(semi, |_| ElementaryTypeName::Byte)(i)
}
fn parse_fixed(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
  named!(semi, tag!(r#"Fixed"#));
  map(semi, |_| ElementaryTypeName::Fixed)(i)
}
fn parse_ufixed(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {
  named!(semi, tag!(r#"Ufixed"#));
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
	    let input = r#"addressa"#;
	    let (remaining, name) = parse_address(input.as_bytes()).ok().unwrap();
	    assert_eq!(
	        (from_utf8(remaining).unwrap(), name),
	        ("a", ElementaryTypeName::Address))
	}
	#[test]
	fn parses_bool() {
	    let input = r#"boola"#;
	    let (remaining, name) = parse_bool(input.as_bytes()).ok().unwrap();
	    assert_eq!(
	        (from_utf8(remaining).unwrap(), name),
	        ("a", ElementaryTypeName::Bool))
	}
	#[test]
	fn parses_string() {
	    let input = r#"stringa"#;
	    let (remaining, name) = parse_string(input.as_bytes()).ok().unwrap();
	    assert_eq!(
	        (from_utf8(remaining).unwrap(), name),
	        ("a", ElementaryTypeName::String))
	}
	#[test]
	fn parses_int() {
	    let input = r#"Inta"#;
	    let (remaining, name) = parse_int(input.as_bytes()).ok().unwrap();
	    assert_eq!(
	        (from_utf8(remaining).unwrap(), name),
	        ("a", ElementaryTypeName::Int))
	}

	#[test]
	fn parses_byte() {
	    let input = r#"Bytea"#;
	    let (remaining, name) = parse_byte(input.as_bytes()).ok().unwrap();
	    assert_eq!(
	        (from_utf8(remaining).unwrap(), name),
	        ("a", ElementaryTypeName::Byte))
	}
	#[test]
	fn parses_fixed() {
	    let input = r#"Fixeda"#;
	    let (remaining, name) = parse_fixed(input.as_bytes()).ok().unwrap();
	    assert_eq!(
	        (from_utf8(remaining).unwrap(), name),
	        ("a", ElementaryTypeName::Fixed))
	}
	#[test]
	fn parses_ufixed() {
	    let input = r#"Ufixeda"#;
	    let (remaining, name) = parse_ufixed(input.as_bytes()).ok().unwrap();
	    assert_eq!(
	        (from_utf8(remaining).unwrap(), name),
	        ("a", ElementaryTypeName::Ufixed))
	}
}
