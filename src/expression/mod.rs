use crate::atom::parse_identifier;
use crate::elementary_type_name::{parse as parse_elementary_type_name, ElementaryTypeName};
use crate::expression::{
    function::{parses_function_call, FunctionCallArguments},
    primary_expr::parse as parse_primary_expression,
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

pub use crate::expression::primary_expr::PrimaryExpression;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    // TODO: PostFix(),
    // TODO: New(),
    // TODO: IndexAccess,
    MemberAccess(Box<Expression>, String),
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
            Expression::MemberAccess(Box::new(exp), mem)
        }),
        map(parses_function_call, |f| {
            let (expr, args) = f;
            Expression::FunctionCall(Box::new(expr), args)
        }),
        delimited(tag("("), parse_expression, tag(")")),
        map(parse_primary_expression, |e| {
            Expression::PrimaryExpression(e)
        }),
    ))(i)
}

pub fn parse_expression_list(i: &[u8]) -> IResult<&[u8], Vec<Expression>> {
    separated_nonempty_list(char(','), preceded(multispace0, parse_expression))(i)
}

fn parse_member_access(i: &[u8]) -> IResult<&[u8], (Expression, String)> {
    map_res(
        separated_pair(take_until("."), char('.'), parse_identifier),
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
    typename: TypeName,
    storage_location: Option<StorageLocation>,
    identifier: Option<String>,
}

pub fn parse_parameter(i: &[u8]) -> IResult<&[u8], Box<Parameter>> {
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
            Box::new(Parameter {
                typename,
                storage_location,
                identifier,
            })
        },
    )(i)
}

pub fn parse_parameter_list(i: &[u8]) -> IResult<&[u8], Vec<Box<Parameter>>> {
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
    fn parses_fully_qualified_parameter() {
        let input = "bool     memory     isWorking\n";
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
        let input = "bool    isWorking\n";
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
        let result = parse_parameter(input.as_bytes());
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, param) = result.ok().unwrap();
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
                    vec![Box::new(Parameter {
                        typename: TypeName::ElementaryTypeName(ElementaryTypeName::Address),
                        storage_location: None,
                        identifier: None
                    })]
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
                        Box::new(Parameter {
                            typename: TypeName::ElementaryTypeName(ElementaryTypeName::Address),
                            storage_location: None,
                            identifier: Some("to".to_string()),
                        }),
                        Box::new(Parameter {
                            typename: TypeName::ElementaryTypeName(ElementaryTypeName::Uint),
                            storage_location: None,
                            identifier: Some("age".to_string()),
                        })
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
                        "bbbb".to_string()
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
                        "b".to_string()
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
                        "bb".to_string()
                    )
                )
            )
        }
    }
}
