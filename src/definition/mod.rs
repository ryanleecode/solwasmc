use crate::atom::{keyword::parse_interface, parse_identifier};
use crate::definition::contract_type::{parse as parse_contract_type, ContractType};
use nom::{
    character::complete::{char, multispace0, multispace1},
    combinator::map,
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

mod constructor;
mod contract_part;
mod contract_type;

#[derive(Debug, PartialEq, Clone)]
pub struct ContractPart {}

fn parse_contract_part(i: &[u8]) -> IResult<&[u8], Box<ContractPart>> {
    panic!("not implemented")
}

#[derive(Debug, PartialEq, Clone)]
pub struct Contract {
    pub contract_type: ContractType,
    pub identifier: String,
    pub contract_part: Vec<Box<ContractPart>>,
}

fn parse_contract(i: &[u8]) -> IResult<&[u8], Box<Contract>> {
    map(
        tuple((
            preceded(multispace1, parse_contract_type),
            preceded(multispace1, parse_identifier),
            terminated(
                preceded(
                    multispace0,
                    preceded(char('{'), preceded(multispace0, many0(parse_contract_part))),
                ),
                preceded(multispace0, char('}')),
            ),
        )),
        |x| {
            let (contract_type, identifier, contract_part) = x;
            Box::new(Contract {
                contract_type,
                identifier,
                contract_part: contract_part,
            })
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use std::str::from_utf8;
    /*
    #[test]
    fn parses_interface_expression() {
        let input = "interface GeneralERC20 { }";
        let (remaining, def) = parse_interface_expression(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), def),
            ("", ContractType::Interface)
        )
    } */
}
