use crate::{
    atom::parse_identifier,
    expression::{parse_expression, parse_type_name, Expression, TypeName},
    storage_location::{parse as parse_storage_location, StorageLocation},
};
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::{complete, map, opt},
    multi::{many0, separated_list},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use std::str::from_utf8;

mod assembly;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub identifier: String,
}

pub fn parse_variable_declaration(i: &[u8]) -> IResult<&[u8], VariableDeclaration> {
    map(
        tuple((
            preceded(multispace0, parse_type_name),
            opt(preceded(multispace1, parse_storage_location)),
            preceded(multispace1, parse_identifier),
        )),
        |x| {
            let (type_name, storage_location, identifier) = x;
            VariableDeclaration {
                type_name,
                storage_location,
                identifier,
            }
        },
    )(i)
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDefinition {
    pub declarations: Vec<VariableDeclaration>,
    pub rhs: Expression,
}

pub fn parse_variable_definition(i: &[u8]) -> IResult<&[u8], VariableDefinition> {
    map(
        tuple((
            alt((
                map(parse_variable_declaration, |x| vec![x]),
                delimited(
                    preceded(multispace0, char('(')),
                    preceded(
                        multispace0,
                        separated_list(char(','), parse_variable_declaration),
                    ),
                    preceded(multispace0, char(')')),
                ),
            )),
            preceded(
                preceded(multispace0, char('=')),
                preceded(multispace0, parse_expression),
            ),
        )),
        |x| {
            let (declarations, rhs) = x;
            VariableDefinition { declarations, rhs }
        },
    )(i)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Block(Vec<Statement>),
    // TODO: IfStatement
    // TODO: WhileStatement
    // TODO: ForStatement
    // TODO: DoWhileStatement
    // TODO: Continue
    // TODO: Break
    // TODO: Return
    // TODO: Throw
    // TODO: Emit
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
    VariableDefinition(VariableDefinition),
}

pub fn parse_statement(i: &[u8]) -> IResult<&[u8], Statement> {
    alt((
        alt((
            map(parse_block, |x| Statement::Block(x)),
            terminated(
                alt((
                    map(parse_expression, |x| Statement::Expression(x)),
                    map(parse_variable_declaration, |x| {
                        Statement::VariableDeclaration(x)
                    }),
                    map(parse_variable_definition, |x| {
                        Statement::VariableDefinition(x)
                    }),
                )),
                preceded(multispace0, tag(";")),
            ),
        )),
        alt((
            map(parse_block, |x| Statement::Block(x)),
            terminated(
                alt((
                    map(parse_variable_declaration, |x| {
                        Statement::VariableDeclaration(x)
                    }),
                    map(parse_variable_definition, |x| {
                        Statement::VariableDefinition(x)
                    }),
                )),
                preceded(multispace0, tag(";")),
            ),
        )),
    ))(i)
}

pub fn parse_block(i: &[u8]) -> IResult<&[u8], Vec<Statement>> {
    delimited(
        preceded(multispace0, tag("{")),
        many0(preceded(multispace0, parse_statement)),
        preceded(multispace0, tag("}")),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::elementary_type_name::ElementaryTypeName;
    use crate::expression::{Expression, PrimaryExpression};
    use crate::literal::Boolean;
    use pretty_assertions::assert_eq;
    use std::str::from_utf8;

    #[test]
    fn parses_variable_declaration() {
        let input = b"     bool     memory         mahmemory";
        let result = parse_variable_declaration(input);
        if result.is_err() {
            result.expect("should parse variable declaration");
        } else {
            let (remaining, decl) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), decl),
                (
                    "",
                    VariableDeclaration {
                        type_name: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                        storage_location: Some(StorageLocation::Memory),
                        identifier: "mahmemory".to_string(),
                    }
                )
            )
        }
    }

    #[test]
    fn parses_with_no_storage_declaration() {
        let input = b"     bool            mahmemory";
        let result = parse_variable_declaration(input);
        if result.is_err() {
            result.expect("should parse variable declaration");
        } else {
            let (remaining, decl) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), decl),
                (
                    "",
                    VariableDeclaration {
                        type_name: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                        storage_location: None,
                        identifier: "mahmemory".to_string(),
                    }
                )
            )
        }
    }

    #[test]
    fn parses_variable_definition() {
        let input = b"    (bool     memory     a,     bool     memory    b   ) =    true";
        let result = parse_variable_definition(input);
        if result.is_err() {
            result.expect("should parse variable definition");
        } else {
            let (remaining, decl) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), decl),
                (
                    "",
                    VariableDefinition {
                        declarations: vec![
                            VariableDeclaration {
                                type_name: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                                storage_location: Some(StorageLocation::Memory),
                                identifier: "a".to_string(),
                            },
                            VariableDeclaration {
                                type_name: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                                storage_location: Some(StorageLocation::Memory),
                                identifier: "b".to_string(),
                            }
                        ],
                        rhs: Expression::PrimaryExpression(PrimaryExpression::BooleanLiteral(
                            Boolean::True
                        ))
                    }
                )
            )
        }
    }

    #[test]
    fn parses_block() {
        let input = b"{(bool     memory     a,     bool     memory    b   ) =    true;}";
        let result = parse_block(input);
        if result.is_err() {
            result.expect("should parse block");
        } else {
            let (remaining, decl) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), decl),
                (
                    "",
                    vec![Statement::VariableDefinition(VariableDefinition {
                        declarations: vec![
                            VariableDeclaration {
                                type_name: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                                storage_location: Some(StorageLocation::Memory),
                                identifier: "a".to_string(),
                            },
                            VariableDeclaration {
                                type_name: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                                storage_location: Some(StorageLocation::Memory),
                                identifier: "b".to_string(),
                            }
                        ],
                        rhs: Expression::PrimaryExpression(PrimaryExpression::BooleanLiteral(
                            Boolean::True
                        ))
                    })]
                )
            )
        }
    }

    #[test]
    fn parses_block2() {
        let input = b"{bool memory a;}";
        let result = parse_block(input);
        if result.is_err() {
            result.expect("should parse block2");
        } else {
            let (remaining, decl) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), decl),
                (
                    "",
                    vec![Statement::VariableDeclaration(VariableDeclaration {
                        type_name: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                        storage_location: Some(StorageLocation::Memory),
                        identifier: "a".to_string(),
                    })]
                )
            )
        }
    }
}
