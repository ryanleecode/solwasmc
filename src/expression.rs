use crate::atom::{keyword::parse_interface, parse_identifier};
use crate::elementary_type_name::ElementaryTypeName;
use crate::storage_location::StorageLocation;
use nom::{
    combinator::map,
    multi::separated_nonempty_list,
    sequence::{delimited, preceded},
    character::complete::{char, multispace0, multispace1},
    IResult,
};

pub type ElementaryTypeNameExpression = ElementaryTypeName;

#[derive(Debug, PartialEq, Clone)]
pub enum ContractDefinition {
    Contract,
    Library,
    Interface,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeName {
    ElementaryTypeName(ElementaryTypeName),
    UserDefinedTypeName,
    // TODO: Mapping
    // TODO: ArrayTypeName
    // TODO: FunctionTypeName
    // TODO: ( 'address' 'payable' )
}

pub fn parse_user_defined_type_name(i: &[u8]) -> IResult<&[u8], TypeName> {
    map(separated_nonempty_list(char('.'), parse_identifier), |_| {
        TypeName::UserDefinedTypeName
    })(i)
}

fn parse_interface_expression(i: &[u8]) -> IResult<&[u8], ContractDefinition> {
    map(
        delimited(
            preceded(parse_interface, preceded(multispace1, parse_identifier)),
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
