use crate::atom::{keyword::parse_interface, parse_identifier};
use crate::definition::contract_type::ContractType;
use nom::{
    character::complete::{char, multispace0, multispace1},
    combinator::map,
    sequence::{delimited, preceded},
    IResult,
};

mod contract_type;

#[derive(Debug, PartialEq, Clone)]
pub struct Contract {
    pub contract_type: ContractType,
}

fn parse_interface_expression(i: &[u8]) -> IResult<&[u8], ContractType> {
    map(
        delimited(
            preceded(parse_interface, preceded(multispace1, parse_identifier)),
            preceded(multispace0, char('{')),
            preceded(multispace0, char('}')),
        ),
        |_| ContractType::Interface,
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
            ("", ContractType::Interface)
        )
    }
}
