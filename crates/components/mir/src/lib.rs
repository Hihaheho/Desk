pub mod block;
pub mod scope;
pub mod stmt;
pub mod var;
pub mod mir;

#[derive(Clone, Debug, PartialEq)]
pub struct FnId(usize);
