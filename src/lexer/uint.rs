#[derive(Debug, PartialEq, Clone)]
pub enum UInt {
	UInt,
	UInt8,
	UInt16,
	UInt24,
	UInt32,
	UInt40,
	UInt48,
	UInt56,
	UInt64,
	UInt72,
	UInt80,
	UInt88,
	UInt96,
	UInt104,
	UInt112,
	UInt120,
	UInt128,
	UInt136,
	UInt144,
	UInt152,
	UInt160,
	UInt168,
	UInt176,
	UInt184,
	UInt192,
	UInt200,
	UInt208,
	UInt216,
	UInt224,
	UInt232,
	UInt240,
	UInt248,
	UInt256,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		lexer::{ElementaryTypeName, UInt},
		solidity,
	};

	use pretty_assertions::assert_eq;

	#[test]
	fn uint8_overrides_uint() {
		let parse_result = solidity::ElementaryTypeNameTermParser::new().parse("uint8");
		assert!(parse_result.is_ok());
		let token = parse_result.ok().unwrap();
		assert_eq!(token, ElementaryTypeName::UInt(UInt::UInt8))
	}
}
