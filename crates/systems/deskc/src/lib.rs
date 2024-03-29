pub mod card;
pub mod error;
pub mod hir_result;
pub mod parse_source_code;
pub mod query_result;

pub use parse_source_code::*;

pub use codebase::code::{Code, SyntaxKind};

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ast::{
        expr::{Expr, Literal},
        meta::{Meta, WithMeta},
    };
    use codebase::code::{Code, SyntaxKind};
    use ids::{Entrypoint, FileId};
    use mir::{
        block::BasicBlock,
        mir::{ControlFlowGraph, ControlFlowGraphId, Mir},
        scope::{Scope, ScopeId},
        stmt::{Const, Stmt, StmtBind, Terminator},
        var::{Var, VarId, Vars},
    };
    use ty::Type;

    use crate::card::{DeskCompiler, DeskcQueries};

    #[test]
    fn compiles_source_code() {
        let mut cards = DeskCompiler::default();
        let file_id = FileId::new();
        cards.set_code(
            file_id.clone(),
            Code::SourceCode {
                syntax: SyntaxKind::Minimalist,
                source: Arc::new("1".into()),
            },
        );
        assert_eq!(
            cards.mir(Entrypoint::File(file_id)).unwrap(),
            Arc::new(Mir {
                entrypoint: ControlFlowGraphId(0),
                cfgs: vec![ControlFlowGraph {
                    parameter: Default::default(),
                    captured: vec![],
                    output: Type::Integer,
                    vars: Vars(vec![Var {
                        ty: Type::Integer,
                        scope: ScopeId(0)
                    },]),
                    scopes: vec![Scope { super_scope: None }],
                    blocks: vec![BasicBlock {
                        stmts: vec![StmtBind {
                            var: VarId(0),
                            stmt: Stmt::Const(Const::Int(1))
                        },],
                        terminator: Terminator::Return(VarId(0))
                    }],
                    links: vec![]
                }]
            })
        );
    }

    #[test]
    fn compiles_ast() {
        let mut cards = DeskCompiler::default();
        let file_id = FileId::new();
        cards.set_code(
            file_id.clone(),
            Code::Ast(Arc::new(WithMeta {
                meta: Meta::new_no_comments(),
                value: Expr::Literal(Literal::Integer(1)),
            })),
        );
        assert_eq!(
            cards.mir(Entrypoint::File(file_id)).unwrap(),
            Arc::new(Mir {
                entrypoint: ControlFlowGraphId(0),
                cfgs: vec![ControlFlowGraph {
                    parameter: Default::default(),
                    captured: vec![],
                    output: Type::Integer,
                    vars: Vars(vec![Var {
                        ty: Type::Integer,
                        scope: ScopeId(0)
                    },]),
                    scopes: vec![Scope { super_scope: None }],
                    blocks: vec![BasicBlock {
                        stmts: vec![StmtBind {
                            var: VarId(0),
                            stmt: Stmt::Const(Const::Int(1))
                        },],
                        terminator: Terminator::Return(VarId(0))
                    }],
                    links: vec![]
                }]
            })
        );
    }
}
