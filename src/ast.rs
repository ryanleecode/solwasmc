use crate::lexer::{ElementaryTypeName, BooleanLiteral};

pub type ElementaryTypeNameExpression = ElementaryTypeName;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    PrimaryExpression(PrimaryExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpression {
    BooleanLiteral(BooleanLiteral),
    Identifier(String),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
}