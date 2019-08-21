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
