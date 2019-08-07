use nom::IResult;
use nom::{delimited};
use nom::sequence::{preceded};
use crate::keyword::parse_interface_keyword;
use nom::character::complete::{multispace0, multispace1, char};
use nom::{
    sequence::{delimited},
    combinator::{map}
};
use std::str::from_utf8;
use crate::atom::parse_identifier;

#[derive(Debug, PartialEq, Clone)]
pub enum ContractDefinition {
    Contract,
    Library,
    Interface,
}

fn parse_interface_expression(i: &[u8]) -> IResult<&[u8], ContractDefinition> {
    map(delimited(
        preceded(parse_interface_keyword, preceded(multispace1, parse_identifier)),
        preceded(multispace0, char('{')),
        preceded(multispace0, char('}')),
    ), |_| ContractDefinition::Interface)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::{assert_eq};

    #[test]
    fn parses_interface_expression() {
        let input = "interface GeneralERC20 { }";
        let (remaining, def) = parse_interface_expression(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), def),
            ("", ContractDefinition::Interface))
    }

}