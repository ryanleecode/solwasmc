use crate::{
    atom::parse_identifier,
    elementary_type_name::ElementaryTypeName,
    expression::{parse_expression, parse_type_name, Expression, TypeName},
    storage_location::{parse as parse_storage_location, StorageLocation},
};
use nom::{
    branch::alt,
    character::complete::{char, multispace0, multispace1},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

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
        map(parse_block, |x| Statement::Block(x)),
        map(parse_expression, |x| Statement::Expression(x)),
    ))(i)
}

pub fn parse_block(i: &[u8]) -> IResult<&[u8], Vec<Statement>> {
    delimited(
        preceded(multispace0, char('{')),
        many0(preceded(multispace0, parse_statement)),
        preceded(multispace0, char('}')),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}