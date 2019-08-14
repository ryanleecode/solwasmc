use crate::atom::parse_identifier;
use crate::expression::{parse_expression, parse_expression_list, Expression};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{char, multispace0},
        is_alphanumeric,
    },
    combinator::{complete, map, map_res, opt},
    multi::separated_list,
    named,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    take_until1, take_while, IResult,
};
use std::str::from_utf8;

pub type NameValue = (String, Expression);

// TODO: Name Value List??
/* fn parse_name_value_list(i: &[u8]) -> IResult<&[u8], Vec<NameValue>> {
    separated_list(char(','), preceded(multispace0, parse_name_value))(i)
}

fn parse_name_value(i: &[u8]) -> IResult<&[u8], NameValue> {
    separated_pair(
        parse_identifier,
        preceded(multispace0, tag(":")),
        preceded(multispace0, parse_expression),
    )(i)
} */

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionCallArguments {
    // NameValueList(Vec<NameValue>),
    ExpressionList(Vec<Expression>),
}

pub fn parses_function_call(i: &[u8]) -> IResult<&[u8], (Expression, FunctionCallArguments)> {
    named!(t1, take_while!(is_alphanumeric));
    complete(map_res(
        tuple((
            terminated(t1, multispace0),
            delimited(tag("("), parse_function_call_arguments, tag(")")),
        )),
        |x| {
            let (expr, args) = x;
            let p_expr = parse_expression(expr);
            if p_expr.is_err() {
                return Err(p_expr.unwrap_err());
            } else {
                let (_, e) = p_expr.unwrap();
                return Ok((e, args));
            }
        },
    ))(i)
}

fn parse_function_call_arguments(i: &[u8]) -> IResult<&[u8], FunctionCallArguments> {
    /*     alt((
           map(delimited(tag("{"), parse_name_value_list, tag("}")), |l| {
               FunctionCallArguments::NameValueList(l)
           }),
           map(opt(parse_expression_list), |l| {
               FunctionCallArguments::ExpressionList(l)
           }),
       ))(i)
    */
    map(opt(parse_expression_list), |l| {
        FunctionCallArguments::ExpressionList(l.unwrap_or(vec![]))
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elementary_type_name::ElementaryTypeName;
    use crate::expression::primary_expr::PrimaryExpression;
    use pretty_assertions::assert_eq;
    use std::str::from_utf8;

    #[test]
    fn parses_name_value() {
        /*  let input = b"a     : bool\n";
        let result = parse_name_value(input);
        if result.is_err() {
            result.expect("should parse name value");
        } else {
            let (remaining, b) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), b),
                (
                    "\n",
                    (
                        "a".to_string(),
                        Expression::PrimaryExpression(
                            PrimaryExpression::ElementaryTypeNameExpression(
                                ElementaryTypeName::Bool
                            )
                        )
                    )
                )
            )
        } */
    }

    #[test]
    fn parses_empty_name_value_list() {
        /* let input = b"";
        let result = parse_name_value_list(input);
        if result.is_err() {
            result.expect("should parse empty name value list");
        } else {
            let (remaining, b) = result.ok().unwrap();
            assert_eq!((from_utf8(remaining).unwrap(), b), ("", vec![]))
        } */
    }

    #[test]
    fn parses_name_value_list() {
        /* let input = b"a: bool, b: bool";
        let result = parse_name_value_list(input);
        if result.is_err() {
            result.expect("should parse name value list");
        } else {
            let (remaining, b) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), b),
                (
                    "",
                    vec![
                        (
                            "a".to_string(),
                            Expression::PrimaryExpression(
                                PrimaryExpression::ElementaryTypeNameExpression(
                                    ElementaryTypeName::Bool
                                )
                            )
                        ),
                        (
                            "b".to_string(),
                            Expression::PrimaryExpression(
                                PrimaryExpression::ElementaryTypeNameExpression(
                                    ElementaryTypeName::Bool
                                )
                            )
                        )
                    ]
                )
            )
        } */
    }
}
