pub mod mir;
pub mod region;
pub mod stmt;
pub mod ty;

pub use amir::amir::AmirId as MirId;
pub use amir::block::BlockId;
pub use amir::link::LinkId;
pub use amir::scope::Scope;
pub use amir::scope::ScopeId;
pub use amir::stmt::Const;
pub use amir::stmt::StmtBind;
pub use amir::stmt::Terminator;
pub use amir::var::VarId;
