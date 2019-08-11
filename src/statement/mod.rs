use crate::{
    expression::{Expression, TypeName},
    storage_location::StorageLocation,
};

mod assembly;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub type_name: TypeName,
    pub storage_location: StorageLocation,
    pub identifier: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDefinition {
    pub declarations: Vec<VariableDeclaration>,
    pub rhs: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Expression),
    // TODO: IfStatement
    // TODO: WhileStatement
    // TODO: ForStatement
    // TODO: DoWhileStatement
    // TODO: Continue
    // TODO: Break
    // TODO: Return
    // TODO: Throw
    // TODO: Emit
    VariableDeclaration(VariableDeclaration),
    VariableDefinition(VariableDefinition),
}
