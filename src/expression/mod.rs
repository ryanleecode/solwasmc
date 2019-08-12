use crate::atom::parse_identifier;
use crate::elementary_type_name::{parse as parse_elementary_type_name, ElementaryTypeName, UInt};
use crate::expression::{
    function::parses_function_call, primary_expr::parse as parse_primary_expression,
};
use crate::storage_location::{parse as parse_storage_location, StorageLocation};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace0, multispace1},
    combinator::{complete, flat_map, map, map_res},
    multi::{separated_list, separated_nonempty_list},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

mod function;
mod primary_expr;

pub use crate::expression::{function::FunctionCallArguments, primary_expr::PrimaryExpression};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    // TODO: PostFix(),
    // TODO: New(),
    // TODO: IndexAccess,
    MemberAccess(Box<Expression>, Box<Expression>),
    FunctionCall(Box<Expression>, FunctionCallArguments),
    // TODO:   ('!' | '~' | 'delete' | '++' | '--' | '+' | '-') Expression
    // TODO: | Expression '**' Expression
    // TODO: | Expression ('*' | '/' | '%') Expression
    // TODO: | Expression ('+' | '-') Expression
    // TODO: | Expression ('<<' | '>>') Expression
    // TODO: | Expression '&' Expression
    // TODO: | Expression '^' Expression
    // TODO: | Expression '|' Expression
    // TODO: | Expression ('<' | '>' | '<=' | '>=') Expression
    // TODO: | Expression ('==' | '!=') Expression
    // TODO: | Expression '&&' Expression
    // TODO: | Expression '||' Expression
    // TODO: | Expression '?' Expression ':' Expression
    // TODO: | Expression ('=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '+=' | '-=' | '*=' | '/=' | '%=') Expression
    // TODO: | PrimaryExpression
    PrimaryExpression(PrimaryExpression),
}

pub fn parse_expression(i: &[u8]) -> IResult<&[u8], Expression> {
    alt((
        map(parse_member_access, |m| {
            let (exp, mem) = m;
            Expression::MemberAccess(Box::new(exp), Box::new(mem))
        }),
        map(parses_function_call, |f| {
            let (expr, args) = f;
            Expression::FunctionCall(Box::new(expr), args)
        }),
        delimited(char('('), parse_expression, char(')')),
        map(parse_primary_expression, |e| {
            Expression::PrimaryExpression(e)
        }),
    ))(i)
}

pub fn parse_expression_list(i: &[u8]) -> IResult<&[u8], Vec<Expression>> {
    separated_nonempty_list(char(','), preceded(multispace0, parse_expression))(i)
}

fn parse_member_access(i: &[u8]) -> IResult<&[u8], (Expression, Expression)> {
    map_res(
        separated_pair(
            take_until("."),
            char('.'),
            alt((
                map(parses_function_call, |x| {
                    let (expr, args) = x;
                    Expression::FunctionCall(Box::new(expr), args)
                }),
                map(parse_identifier, |x| {
                    Expression::PrimaryExpression(PrimaryExpression::Identifier(x))
                }),
            )),
        ),
        |x| {
            let (expr, id) = x;
            let p_expr = parse_expression(expr);
            if p_expr.is_err() {
                return Err(p_expr.unwrap_err());
            } else {
                let (_, e) = p_expr.unwrap();
                return Ok((e, id));
            }
        },
    )(i)
}

pub type ElementaryTypeNameExpression = ElementaryTypeName;

#[derive(Debug, PartialEq, Clone)]
pub enum TypeName {
    ElementaryTypeName(ElementaryTypeName),
    UserDefinedTypeName(Vec<String>),
    // TODO: Mapping
    // TODO: ArrayTypeName
    // TODO: FunctionTypeName
    // TODO: ( 'address' 'payable' )
}

pub fn parse_user_defined_type_name(i: &[u8]) -> IResult<&[u8], TypeName> {
    map(separated_nonempty_list(char('.'), parse_identifier), |x| {
        TypeName::UserDefinedTypeName(x)
    })(i)
}

pub fn parse_type_name(i: &[u8]) -> IResult<&[u8], TypeName> {
    alt((
        map(parse_elementary_type_name, |e| {
            TypeName::ElementaryTypeName(e)
        }),
        parse_user_defined_type_name,
    ))(i)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub typename: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub identifier: Option<String>,
}

use std::str::from_utf8;
pub fn parse_parameter(i: &[u8]) -> IResult<&[u8], Parameter> {
    map(
        tuple((
            parse_type_name,
            alt((
                complete(map(
                    tuple((
                        preceded(multispace1, parse_storage_location),
                        preceded(multispace1, parse_identifier),
                    )),
                    |tup| {
                        let (storage, id) = tup;
                        (Some(storage), Some(id.to_string()))
                    },
                )),
                complete(map(preceded(multispace1, parse_identifier), |id| {
                    (None, Some(id.to_string()))
                })),
                map(multispace0, move |_| (None, None)),
            )),
        )),
        |t| {
            let (typename, params) = t;
            let (storage_location, identifier) = params;
            Parameter {
                typename,
                storage_location,
                identifier,
            }
        },
    )(i)
}

pub fn parse_parameter_list(i: &[u8]) -> IResult<&[u8], Vec<Parameter>> {
    terminated(
        preceded(
            char('('),
            preceded(
                multispace0,
                separated_list(
                    preceded(multispace0, char(',')),
                    preceded(multispace0, parse_parameter),
                ),
            ),
        ),
        preceded(multispace0, char(')')),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::elementary_type_name::ElementaryTypeName;
    use crate::expression::function::FunctionCallArguments;
    use crate::literal::Number;
    use pretty_assertions::assert_eq;
    use std::str::from_utf8;

    #[test]
    fn parses_user_defined_type_name() {
        let input = "OpenZepp.ERC20.ABC {";
        let (remaining, typename) = parse_user_defined_type_name(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), typename),
            (
                " {",
                TypeName::UserDefinedTypeName(vec![
                    "OpenZepp".to_string(),
                    "ERC20".to_string(),
                    "ABC".to_string()
                ])
            )
        )
    }

    #[test]
    fn parses_user_defined_type_name_with_no_periods() {
        let input = "keccak";
        let (remaining, typename) = parse_user_defined_type_name(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), typename),
            (
                "",
                TypeName::UserDefinedTypeName(vec!["keccak".to_string(),])
            )
        )
    }

    #[test]
    fn parses_fully_qualified_parameter() {
        let input = "bool     memory     isWorking\n";
        let (remaining, param) = parse_parameter(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), param),
            (
                "\n",
                Parameter {
                    typename: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                    storage_location: Some(StorageLocation::Memory),
                    identifier: Some("isWorking".to_string()),
                }
            )
        )
    }

    #[test]
    fn parses_parameter_with_identifier_only() {
        let input = "bool    isWorking\n";
        let (remaining, param) = parse_parameter(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), param),
            (
                "\n",
                Parameter {
                    typename: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                    storage_location: None,
                    identifier: Some("isWorking".to_string()),
                }
            )
        )
    }

    #[test]
    fn parse_parameter_tolerate_uint256() {
        let input = "uint256 value";
        let (remaining, param) = parse_parameter(input.as_bytes()).ok().unwrap();
        assert_eq!(
            (from_utf8(remaining).unwrap(), param),
            (
                "",
                Parameter {
                    typename: TypeName::ElementaryTypeName(ElementaryTypeName::UInt(UInt::Uint256)),
                    storage_location: None,
                    identifier: Some("value".to_string()),
                }
            )
        )
    }

    #[test]
    fn parses_parameter_with_type_only() {
        let input = "bool   \n";
        let result = parse_parameter(input.as_bytes());
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, param) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), param),
                (
                    "",
                    Parameter {
                        typename: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
                        storage_location: None,
                        identifier: None,
                    }
                )
            )
        }
    }

    #[test]
    fn parses_parameter_list_no_params() {
        let input = "(    )";
        let result = parse_parameter_list(input.as_bytes());
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, params) = result.ok().unwrap();
            assert_eq!((from_utf8(remaining).unwrap(), params), ("", Vec::new()))
        }
    }

    #[test]
    fn parses_parameter_list_one_param() {
        let input = "(   address   )";
        let result = parse_parameter_list(input.as_bytes());
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, params) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), params),
                (
                    "",
                    vec![Parameter {
                        typename: TypeName::ElementaryTypeName(ElementaryTypeName::Address),
                        storage_location: None,
                        identifier: None
                    }]
                )
            )
        }
    }

    #[test]
    fn parses_parameter_list_mutliple_params() {
        let input = "(address   to   ,   uint   age)";
        let result = parse_parameter_list(input.as_bytes());
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, params) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), params),
                (
                    "",
                    vec![
                        Parameter {
                            typename: TypeName::ElementaryTypeName(ElementaryTypeName::Address),
                            storage_location: None,
                            identifier: Some("to".to_string()),
                        },
                        Parameter {
                            typename: TypeName::ElementaryTypeName(ElementaryTypeName::UInt(
                                UInt::Uint
                            )),
                            storage_location: None,
                            identifier: Some("age".to_string()),
                        }
                    ]
                )
            )
        }
    }

    #[test]
    fn parses_member_access() {
        let input = "aaaa.bbbb\n";
        let result = parse_member_access(input.as_bytes());
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, params) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), params),
                (
                    "\n",
                    (
                        Expression::PrimaryExpression(PrimaryExpression::Identifier(
                            "aaaa".to_string()
                        )),
                        Expression::PrimaryExpression(PrimaryExpression::Identifier(
                            "bbbb".to_string()
                        ))
                    )
                )
            )
        }
    }

    #[test]
    fn parses_member_access2() {
        let input = "aa.b\n";
        let result = parse_member_access(input.as_bytes());
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, params) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), params),
                (
                    "\n",
                    (
                        Expression::PrimaryExpression(PrimaryExpression::Identifier(
                            "aa".to_string()
                        )),
                        Expression::PrimaryExpression(PrimaryExpression::Identifier(
                            "b".to_string()
                        ))
                    )
                )
            )
        }
    }

    #[test]
    fn parses_member_access3() {
        let input = "a.bb\n";
        let result = parse_member_access(input.as_bytes());
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, params) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), params),
                (
                    "\n",
                    (
                        Expression::PrimaryExpression(PrimaryExpression::Identifier(
                            "a".to_string()
                        )),
                        Expression::PrimaryExpression(PrimaryExpression::Identifier(
                            "bb".to_string()
                        ))
                    )
                )
            )
        }
    }

    #[test]
    fn parses_fnc_call_with_member_access_fnc_call() {
        let input = "GeneralERC20(0xf25186B5081Ff5cE73482AD761DB0eB0d25abfBF).transfer(0x821aEa9a577a9b44299B9c15c88cf3087F3b5544, 250)\n";
        let result = parse_member_access(input.as_bytes());
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, params) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), params),
                (
                    "\n",
                    (
                        Expression::FunctionCall(
                            Box::new(Expression::PrimaryExpression(
                                PrimaryExpression::Identifier("GeneralERC20".to_string())
                            )),
                            FunctionCallArguments::ExpressionList(Some(vec![
                                Expression::PrimaryExpression(PrimaryExpression::NumberLiteral((
                                    Number::Hex(
                                        "0xf25186B5081Ff5cE73482AD761DB0eB0d25abfBF".to_string()
                                    ),
                                    None
                                )))
                            ]))
                        ),
                        Expression::FunctionCall(
                            Box::new(Expression::PrimaryExpression(
                                PrimaryExpression::Identifier("transfer".to_string())
                            )),
                            FunctionCallArguments::ExpressionList(Some(vec![
                                Expression::PrimaryExpression(PrimaryExpression::NumberLiteral((
                                    Number::Hex(
                                        "0x821aEa9a577a9b44299B9c15c88cf3087F3b5544".to_string()
                                    ),
                                    None
                                ))),
                                Expression::PrimaryExpression(PrimaryExpression::NumberLiteral((
                                    Number::Decimal("250".to_string()),
                                    None
                                )))
                            ]))
                        ),
                    )
                )
            )
        }
    }
}
