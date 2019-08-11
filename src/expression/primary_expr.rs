use crate::atom::parse_identifier;
use crate::elementary_type_name::{parse as parse_elementary_type_name, ElementaryTypeName};
use crate::literal::{parse as parse_literal, Boolean, Literal, NumberLiteral};
use nom::{
    branch::alt,
    combinator::{map, map_res},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpression {
    BooleanLiteral(Boolean),
    NumberLiteral(NumberLiteral),
    // TODO: HexLiteral
    // TODO Tuple Expression
    Identifier(String),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
}

pub type ElementaryTypeNameExpression = ElementaryTypeName;

pub fn parse(i: &[u8]) -> IResult<&[u8], PrimaryExpression> {
    alt((
        map_res(parse_literal, |l| match l {
            Literal::Boolean(b) => Ok(PrimaryExpression::BooleanLiteral(b)),
            Literal::Number(n) => Ok(PrimaryExpression::NumberLiteral(n)),
            _ => Err("not a primary expression"),
        }),
        map(parse_elementary_type_name, |n| {
            PrimaryExpression::ElementaryTypeNameExpression(n)
        }),
        map(parse_identifier, |id| PrimaryExpression::Identifier(id)),
    ))(i)
}
