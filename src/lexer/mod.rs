pub use self::{byte::Byte, uint::UInt};

mod byte;
mod uint;

#[derive(Debug, PartialEq, Clone)]
pub enum ElementaryTypeName {
    Address,
    Bool,
    String,
    UInt(UInt),
    Byte(Byte),
}
