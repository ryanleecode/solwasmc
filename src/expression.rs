use crate::atom::{keyword::parse_interface, parse_identifier};
use crate::elementary_type_name::{parse as parse_elementary_type_name, ElementaryTypeName};
use crate::storage_location::{parse as parse_storage_location, StorageLocation};
use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete::{char, multispace0, multispace1},
    combinator::{flat_map, map, opt, complete},
    multi::separated_nonempty_list,
    sequence::{delimited, preceded, tuple},
    IResult,
};

pub type ElementaryTypeNameExpression = ElementaryTypeName;

#[derive(Debug, PartialEq, Clone)]
pub enum ContractDefinition {
    Contract,
    Library,
    Interface,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

pub fn parse_type_name(i: &[u8]) -> IResult<&[u8], TypeName> {
    alt((
        map(parse_elementary_type_name, |e| {
            // println!("{:#?}", TypeName::ElementaryTypeName(e.clone()));
            TypeName::ElementaryTypeName(e)
        }),
        parse_user_defined_type_name,
    ))(i)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    typename: TypeName,
    storage_location: Option<StorageLocation>,
    identifier: Option<String>,
}

pub fn parse_parameter(i: &[u8]) -> IResult<&[u8], Box<Parameter>> {
    flat_map(parse_type_name, |typename| {
        alt((
            complete(map(
                tuple((
                    preceded(multispace1, parse_storage_location),
                    preceded(multispace1, parse_identifier),
                )),
                move |tup| {
                    let (storage, id) = tup;
                    Box::new(Parameter {
                        typename: typename,
                        storage_location: Some(storage),
                        identifier: Some(id.to_string()),
                    })
                },
            )),
            complete(map(preceded(multispace1, parse_identifier), move |id| {
                Box::new(Parameter {
                    typename: typename,
                    storage_location: None,
                    identifier: Some(id.to_string()),
                })
            })),
            map(multispace0, move |_| {
                Box::new(Parameter {
                    typename: typename,
                    storage_location: None,
                    identifier: None,
                })
            }),
        ))
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

    #[test]
    fn parses_fully_qualified_parameter() {
        let input = "bool memory isWorking\n";
        let (remaining, param) = parse_parameter(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), param),
            (
                "\n",
                Box::new(Parameter {
                    typename: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                    storage_location: Some(StorageLocation::Memory),
                    identifier: Some("isWorking".to_string()),
                })
            )
        )
    }

    #[test]
    fn parses_parameter_with_identifier_only() {
        let input = "bool isWorking\n";
        let (remaining, param) = parse_parameter(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), param),
            (
                "\n",
                Box::new(Parameter {
                    typename: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                    storage_location: None,
                    identifier: Some("isWorking".to_string()),
                })
            )
        )
    }

    #[test]
    fn parses_parameter_with_type_only() {
        let input = "bool   \n";
        let (remaining, param) = parse_parameter(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), param),
            (
                "",
                Box::new(Parameter {
                    typename: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                    storage_location: None,
                    identifier: None,
                })
            )
        )
    }
}
