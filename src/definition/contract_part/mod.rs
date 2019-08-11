use crate::{
    atom::parse_identifier,
    expression::{parse_expression, parse_type_name, Expression, Parameter, TypeName},
    state_mutability::StateMutability,
    visibility::Visibility,
};
use nom::{
    character::complete::{char, multispace0, multispace1},
    combinator::map,
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

pub fn parse_state_variable_declaration(i: &[u8]) -> IResult<&[u8], Box<StateVariableDeclaration>> {
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
            Box::new(StateVariableDeclaration {
                type_name,
                identifier,
                rhs: expr,
            })
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
    pub identifier: String,
    pub visibility: Visibility,
    pub state_mutability: Option<StateMutability>,
    pub parameter_list: Vec<Parameter>,
    /*     pub block: Option<> */
}

// TODO: EventDefinition
#[derive(Debug, PartialEq, Clone)]
pub struct EventDefinition {}

// TODO: EnumDefinition
#[derive(Debug, PartialEq, Clone)]
pub struct EnumDefinition {}

#[derive(Debug, PartialEq, Clone)]
pub enum ContractPart {
    StateVariableDeclaration(StateVariableDeclaration),
    UsingForDeclaration(UsingForDeclaration),
    StructDefinition(StructDefinition),
    ModifierDefinition(ModifierDefinition),
    ModifierInvocation(ModifierInvocation),
    FunctionDefinition(FunctionDefinition),
    EventDefinition(EventDefinition),
    EnumDefinition(EnumDefinition),
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{elementary_type_name::ElementaryTypeName, expression::PrimaryExpression};
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
                    Box::new(StateVariableDeclaration {
                        type_name: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                        identifier: "a".to_string(),
                        rhs: Expression::MemberAccess(
                            Box::new(Expression::PrimaryExpression(
                                PrimaryExpression::Identifier("b".to_string())
                            )),
                            "c".to_string()
                        )
                    })
                )
            )
        }
    }
}
