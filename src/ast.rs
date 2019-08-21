use crate::lexer::{BooleanLiteral, ElementaryTypeName, Operator};

pub type UserDefinedTypeName = Vec<String>;

pub type ElementaryTypeNameExpression = ElementaryTypeName;

#[derive(Debug, PartialEq, Clone)]
pub enum TypeName {
    ElementaryTypeName(ElementaryTypeName),
    UserDefinedTypeName(UserDefinedTypeName),
    Mapping(ElementaryTypeName, Box<TypeName>),
}

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
        lexer::{Operator, UInt},
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

    #[test]
    fn parses_user_defined_type_name() {
        let parse_result = solidity::UserDefinedTypeNameParser::new().parse("asdf.qwer");
        assert!(parse_result.is_ok());
        let expr = parse_result.ok().unwrap();
        assert_eq!(expr, (vec!["asdf".to_string(), "qwer".to_string()]));
    }

    #[test]
    fn parses_mapping() {
        let parse_result = solidity::TypeNameParser::new().parse("mapping(address => uint)");
        assert!(parse_result.is_ok());
        let expr = parse_result.ok().unwrap();
        assert_eq!(
            expr,
            TypeName::Mapping(
                ElementaryTypeName::Address,
                Box::new(TypeName::ElementaryTypeName(ElementaryTypeName::UInt(
                    UInt::UInt,
                )))
            )
        );
    }
}
