use crate::atom::{
  Atom,
  keyword::{parse_pragma},
  delimiter::{parse_semicolon, Delimiter},
  parse_anything_till_semi,
  parse_identifier,
};
use nom::combinator::map;
use nom::{named, ws, IResult};

#[derive(Debug, PartialEq, Clone)]
pub struct PragmaDirective {
  pub name: String,
  pub value: String,
}

fn parse_pragma_directive(i: &[u8]) -> IResult<&[u8], Box<PragmaDirective>> {
  named!(tuple<&[u8], (Atom, Atom, Atom, Delimiter)>, ws!(tuple!(
        parse_pragma, parse_identifier, parse_anything_till_semi, parse_semicolon)));
  map(tuple, |t| {
    let (_, identifier, version, _) = t;
    Box::new(PragmaDirective {
      name: identifier.to_string(),
      value: version.to_string(),
    })
  })(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::str::from_utf8;
  use pretty_assertions::assert_eq;

  #[test]
  fn parses_pragma_directive() {
    let input = "pragma solidity ^0.5.6;";
    let (remaining, directive) = parse_pragma_directive(input.as_bytes()).ok().unwrap();
    assert_eq!(
      (from_utf8(remaining).unwrap(), directive.as_ref()),
      (
        "",
        &PragmaDirective {
          name: "solidity".to_string(),
          value: "^0.5.6".to_string()
        }
      )
    )
  }
}
