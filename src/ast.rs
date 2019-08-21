use crate::lexer::{BooleanLiteral, ElementaryTypeName, Operator};

pub type ElementaryTypeNameExpression = ElementaryTypeName;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    PostfixExpression(Box<Expression>, Operator),
    PrimaryExpression(PrimaryExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpression {
    BooleanLiteral(BooleanLiteral),
    Identifier(String),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    use crate::{
        ast::{Expression, PrimaryExpression},
        lexer::Operator,
        solidity,
    };

    #[test]
    fn parses_postfix_increment_operator() {
        let parse_result = solidity::ExpressionParser::new().parse("a++");
        assert!(parse_result.is_ok());
        let expr = parse_result.ok().unwrap();
        assert_eq!(
            expr,
            Box::new(Expression::PostfixExpression(
                Box::new(Expression::PrimaryExpression(
                    PrimaryExpression::Identifier("a".to_string())
                )),
                Operator::Increment,
            ))
        )
    }

    #[test]
    fn parses_postfix_decrement_operator() {
        let parse_result = solidity::ExpressionParser::new().parse("a--");
        assert!(parse_result.is_ok());
        let expr = parse_result.ok().unwrap();
        assert_eq!(
            expr,
            Box::new(Expression::PostfixExpression(
                Box::new(Expression::PrimaryExpression(
                    PrimaryExpression::Identifier("a".to_string())
                )),
                Operator::Decrement,
            ))
        )
    }
}
