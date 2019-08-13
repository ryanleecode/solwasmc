use crate::{
    expression::{parse_parameter_list, Parameter, TypeName},
    statement::{parse_block, Statement, VariableDeclaration, VariableDefinition},
    visibility::{parse as parse_visibility, Visibility},
};
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{complete, map, opt},
    dbg_dmp,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Constructor {
    pub parameter_list: Vec<Parameter>,
    pub visibility: Option<Visibility>,
    pub statements: Vec<Statement>,
}

use std::str::from_utf8;
pub fn parse(i: &[u8]) -> IResult<&[u8], Constructor> {
    map(
        tuple((
            preceded(multispace0, complete(tag("constructor"))),
            preceded(multispace0, parse_parameter_list),
            opt(preceded(multispace0, parse_visibility)),
            preceded(multispace0, parse_block),
        )),
        |x| {
            let (_, parameter_list, visibility, statements) = x;
            Constructor {
                parameter_list,
                statements,
                visibility,
            }
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elementary_type_name::ElementaryTypeName;
    use crate::expression::{Expression, FunctionCallArguments, PrimaryExpression, TypeName};
    use crate::literal::Number;
    use pretty_assertions::assert_eq;
    use std::str::from_utf8;

    #[test]
    fn parses_constructor() {
        let input = b"constructor(address lol) public \n{\taddress to = address(0xFB88dE099e13c3ED21F80a7a1E49f8CAEcF10df6); \n}";
        let result = parse(input);
        if result.is_err() {
            result.expect("should parse constructor");
        } else {
            let (remaining, ctor) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), ctor),
                (
                    "",
                    Constructor {
                        parameter_list: vec![Parameter {
                            typename: TypeName::ElementaryTypeName(ElementaryTypeName::Address),
                            storage_location: None,
                            identifier: Some("lol".to_string())
                        }],
                        visibility: Some(Visibility::Public),
                        statements: vec![Statement::VariableDefinition(VariableDefinition {
                            declarations: vec![VariableDeclaration {
                                type_name: TypeName::ElementaryTypeName(
                                    ElementaryTypeName::Address
                                ),
                                storage_location: None,
                                identifier: "to".to_string(),
                            },],
                            rhs: Expression::FunctionCall(
                                Box::new(Expression::PrimaryExpression(
                                    PrimaryExpression::ElementaryTypeNameExpression(
                                        ElementaryTypeName::Address
                                    )
                                )),
                                FunctionCallArguments::ExpressionList(Some(vec![
                                    Expression::PrimaryExpression(
                                        PrimaryExpression::NumberLiteral((
                                            Number::Hex(
                                                "0xFB88dE099e13c3ED21F80a7a1E49f8CAEcF10df6"
                                                    .to_string()
                                            ),
                                            None
                                        ))
                                    )
                                ]))
                            )
                        })],
                    }
                )
            )
        }
    }
}

/*
Statement::Expression(Expression::FunctionCall(
                            Box::new(Expression::PrimaryExpression(
                                PrimaryExpression::ElementaryTypeNameExpression(
                                    ElementaryTypeName::Address
                                )
                            )),
                            FunctionCallArguments::ExpressionList(Some(vec![
                                Expression::PrimaryExpression(PrimaryExpression::NumberLiteral((
                                    Number::Hex(
                                        "0xFB88dE099e13c3ED21F80a7a1E49f8CAEcF10df6".to_string()
                                    ),
                                    None
                                )))
                            ]))
                        )) */
