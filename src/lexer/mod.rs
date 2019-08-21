pub use self::{byte::Byte, operator::Operator, uint::UInt};

mod byte;
mod operator;
mod uint;

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Hex(String),
    Decimal(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BooleanLiteral {
    True,
    False,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElementaryTypeName {
    Address,
    Bool,
    String,
    UInt(UInt),
    Byte(Byte),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solidity;

    use pretty_assertions::assert_eq;

    #[test]
    fn parses_hex_literal_double_quotes() {
        let parse_result = solidity::HexLiteralParser::new().parse("hex\"123456\"");
        assert!(parse_result.is_ok());
        let literal = parse_result.ok().unwrap();
        assert_eq!(literal, "123456".to_string())
    }

    #[test]
    fn parses_hex_literal_single_quotes() {
        let parse_result = solidity::HexLiteralParser::new().parse("hex'AdEc36'");
        assert!(parse_result.is_ok());
        let literal = parse_result.ok().unwrap();
        assert_eq!(literal, "AdEc36".to_string())
    }

    #[test]
    fn parses_string_literal() {
        let parse_result = solidity::StringLiteralParser::new().parse(r#""asdf\"fdsa""#);
        assert!(parse_result.is_ok());
        let literal = parse_result.ok().unwrap();
        assert_eq!(literal, "asdf\\\"fdsa".to_string())
    }
}
