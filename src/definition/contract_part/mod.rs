use crate::{
    atom::parse_identifier,
    definition::constructor::{parse as parse_constructor, Constructor},
    expression::{
        parse_expression, parse_parameter_list, parse_type_name, Expression, Parameter, TypeName,
    },
    state_mutability::StateMutability,
    statement::{parse_block, Statement},
    visibility::{parse as parse_visibility, Visibility},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::{complete, map, opt},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct StateVariableDeclaration {
    pub type_name: TypeName,
    // TODO: pub visibility: 'public' | 'internal' | 'private' | 'constant',
    pub identifier: String,
    pub rhs: Expression,
}

pub fn parse_state_variable_declaration(i: &[u8]) -> IResult<&[u8], StateVariableDeclaration> {
    map(
        tuple((
            preceded(multispace0, parse_type_name),
            preceded(multispace1, parse_identifier),
            preceded(
                multispace1,
                preceded(
                    char('='),
                    preceded(
                        multispace0,
                        terminated(parse_expression, preceded(multispace0, char(';'))),
                    ),
                ),
            ),
        )),
        |d| {
            let (type_name, identifier, expr) = d;
            StateVariableDeclaration {
                type_name,
                identifier,
                rhs: expr,
            }
        },
    )(i)
}

// TODO: UsingForDeclaration
#[derive(Debug, PartialEq, Clone)]
pub struct UsingForDeclaration {}

// TODO: StructDefinition
#[derive(Debug, PartialEq, Clone)]
pub struct StructDefinition {}

// TODO: ModifierDefinition
#[derive(Debug, PartialEq, Clone)]
pub struct ModifierDefinition {}

// TODO: ModifierInvocation
#[derive(Debug, PartialEq, Clone)]
pub struct ModifierInvocation {}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefinition {
    pub identifier: Option<String>,
    pub visibility: Option<Visibility>,
    pub state_mutability: Option<StateMutability>,
    pub parameter_list: Vec<Parameter>,
    pub returns: Vec<Parameter>,
    pub block: Vec<Statement>,
}

use std::str::from_utf8;
fn parse_function_definition(i: &[u8]) -> IResult<&[u8], FunctionDefinition> {
    map(
        preceded(
            preceded(multispace0, complete(tag("function"))),
            tuple((
                opt(preceded(multispace1, parse_identifier)),
                preceded(multispace0, parse_parameter_list),
                opt(preceded(multispace0, parse_visibility)),
                opt(preceded(
                    multispace1,
                    preceded(
                        complete(tag("returns")),
                        preceded(multispace1, parse_parameter_list),
                    ),
                )),
                alt((
                    map(preceded(multispace0, char(';')), |_| Vec::new()),
                    parse_block,
                )),
            )),
        ),
        |x| {
            let (identifier, parameter_list, visibility, returns, block) = x;
            FunctionDefinition {
                identifier,
                parameter_list,
                visibility,
                state_mutability: None,
                returns: returns.unwrap_or(Vec::new()),
                block,
            }
        },
    )(i)
}

// TODO: EventDefinition
#[derive(Debug, PartialEq, Clone)]
pub struct EventDefinition {}

// TODO: EnumDefinition
#[derive(Debug, PartialEq, Clone)]
pub struct EnumDefinition {}

#[derive(Debug, PartialEq, Clone)]
pub enum ContractPart {
    ConstructorDefinition(Constructor),
    StateVariableDeclaration(StateVariableDeclaration),
    UsingForDeclaration(UsingForDeclaration),
    StructDefinition(StructDefinition),
    ModifierDefinition(ModifierDefinition),
    ModifierInvocation(ModifierInvocation),
    FunctionDefinition(FunctionDefinition),
    EventDefinition(EventDefinition),
    EnumDefinition(EnumDefinition),
}

pub fn parse(i: &[u8]) -> IResult<&[u8], ContractPart> {
    alt((
        map(parse_constructor, |x| {
            ContractPart::ConstructorDefinition(x)
        }),
        map(parse_state_variable_declaration, |x| {
            ContractPart::StateVariableDeclaration(x)
        }),
        map(parse_function_definition, |x| {
            ContractPart::FunctionDefinition(x)
        }),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        elementary_type_name::{uint::UInt, ElementaryTypeName},
        expression::PrimaryExpression,
        visibility::Visibility,
    };
    use pretty_assertions::assert_eq;
    use std::str::from_utf8;

    #[test]
    fn parses_state_variable_declaration() {
        let input = b"bool a = b.c;";
        let result = parse_state_variable_declaration(input);
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, declaration) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), declaration),
                (
                    "",
                    StateVariableDeclaration {
                        type_name: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                        identifier: "a".to_string(),
                        rhs: Expression::MemberAccess(
                            Box::new(Expression::PrimaryExpression(
                                PrimaryExpression::Identifier("b".to_string())
                            )),
                            "c".to_string()
                        )
                    }
                )
            )
        }
    }

    #[test]
    fn parses_function_declaration() {
        let input = b"function transfer(address to, uint256 value) external;";
        let result = parse_function_definition(input);
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, declaration) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), declaration),
                (
                    "",
                    FunctionDefinition {
                        identifier: Some("transfer".to_string()),
                        visibility: Some(Visibility::External),
                        state_mutability: None,
                        parameter_list: vec![
                            Parameter {
                                typename: TypeName::ElementaryTypeName(ElementaryTypeName::Address),
                                storage_location: None,
                                identifier: Some("to".to_string())
                            },
                            Parameter {
                                typename: TypeName::ElementaryTypeName(ElementaryTypeName::UInt(
                                    UInt::Uint256
                                )),
                                storage_location: None,
                                identifier: Some("value".to_string())
                            }
                        ],
                        returns: vec![],
                        block: vec![],
                    }
                )
            )
        }
    }
}
