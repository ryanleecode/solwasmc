use crate::lexer::{BooleanLiteral, ElementaryTypeName, Number, NumberUnit, Operator};

pub type UserDefinedTypeName = Vec<String>;

pub type ElementaryTypeNameExpression = ElementaryTypeName;

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionCallArguments {
    NameValueList(Option<Vec<(String, Box<Expression>)>>),
    ExpressionList(Option<Vec<Box<Expression>>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeName {
    Elementary(ElementaryTypeName),
    UserDefined(UserDefinedTypeName),
    Mapping(ElementaryTypeName, Box<TypeName>),
    Array(Box<TypeName>, Option<Box<Expression>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Postfix(Box<Expression>, Operator),
    New(TypeName),
    MemberAccess(Box<Expression>, String),
    IndexAccess(Box<Expression>, Option<Box<Expression>>),
    FunctionCall(Box<Expression>, FunctionCallArguments),
    Prefix(Operator, Box<Expression>),
    MidOp(Box<Expression>, Operator, Box<Expression>),
    Primary(PrimaryExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpression {
    BooleanLiteral(BooleanLiteral),
    NumberLiteral(Number, Option<NumberUnit>),
    HexLiteral(String),
    StringLiteral(String),
    TupleExpression(Vec<Expression>),
    Identifier(String),
    ElementaryTypeName(ElementaryTypeNameExpression),
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
            Box::new(Expression::Postfix(
                Box::new(Expression::Primary(PrimaryExpression::Identifier(
                    "a".to_string()
                ))),
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
            Box::new(Expression::Postfix(
                Box::new(Expression::Primary(PrimaryExpression::Identifier(
                    "a".to_string()
                ))),
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
                Box::new(TypeName::Elementary(ElementaryTypeName::UInt(UInt::UInt,)))
            )
        );
    }

    #[test]
    fn parses_array_typename() {
        let parse_result = solidity::TypeNameParser::new().parse("a[]");
        assert!(parse_result.is_ok());
        let expr = parse_result.ok().unwrap();
        assert_eq!(
            expr,
            TypeName::Array(Box::new(TypeName::UserDefined(vec!["a".to_string()])), None,)
        );
    }
}
