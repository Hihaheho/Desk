pub mod mir;
pub mod region;
pub mod stmt;
pub mod ty;

pub use amir::amir::AmirId as MirId;
pub use amir::block::BlockId;
pub use amir::scope::Scope;
pub use amir::scope::ScopeId;
pub use amir::stmt::ATerminator;
pub use amir::stmt::Const;
pub use amir::stmt::Op;
pub use amir::stmt::StmtBind;
pub use amir::var::VarId;
pub use amir::var::Vars;
