use crate::atom::parse_identifier;
use crate::keyword::parse_interface;
use crate::elementary_type_name::ElementaryTypeName;
use nom::character::complete::{char, multispace0, multispace1};
use nom::sequence::preceded;
use nom::IResult;
use nom::{combinator::map, sequence::delimited};

pub type ElementaryTypeNameExpression = ElementaryTypeName;

#[derive(Debug, PartialEq, Clone)]
pub enum ContractDefinition {
  Contract,
  Library,
  Interface,
}

fn parse_interface_expression(i: &[u8]) -> IResult<&[u8], ContractDefinition> {
  map(
    delimited(
      preceded(
        parse_interface,
        preceded(multispace1, parse_identifier),
      ),
      preceded(multispace0, char('{')),
      preceded(multispace0, char('}')),
    ),
    |_| ContractDefinition::Interface,
  )(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;
  use std::str::from_utf8;

  #[test]
  fn parses_interface_expression() {
    let input = "interface GeneralERC20 { }";
    let (remaining, def) = parse_interface_expression(input.as_bytes()).ok().unwrap();
    assert_eq!(
      (from_utf8(remaining).unwrap(), def),
      ("", ContractDefinition::Interface)
    )
  }

}
