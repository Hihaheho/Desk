mod card;
mod parse_source_code;
mod query_result;

pub use parse_source_code::*;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ast::{
        expr::{Expr, Literal},
        span::WithSpan,
    };
    use codebase::code::{Code, SyntaxKind};
    use ids::{CardId, NodeId};
    use mir::{
        block::BasicBlock,
        mir::{ControlFlowGraph, ControlFlowGraphId, Mir},
        scope::{Scope, ScopeId},
        stmt::{Const, Stmt, StmtBind, Terminator},
        var::{Var, VarId, Vars},
    };
    use types::Type;

    use crate::card::{CardQueries, CardsCompiler};

    #[test]
    fn compiles_source_code() {
        let mut cards = CardsCompiler::default();
        let card_id = CardId::new();
        cards.set_code(
            card_id.clone(),
            Code::SourceCode {
                syntax: SyntaxKind::Hacker,
                source: Arc::new("1".into()),
            },
        );
        assert_eq!(
            cards.mir(card_id).unwrap(),
            Arc::new(Mir {
                entrypoint: ControlFlowGraphId(0),
                cfgs: vec![ControlFlowGraph {
                    parameters: vec![],
                    captured: vec![],
                    output: Type::Number,
                    vars: Vars(vec![Var {
                        ty: Type::Number,
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
        let mut cards = CardsCompiler::default();
        let card_id = CardId::new();
        cards.set_code(
            card_id.clone(),
            Code::Ast(Arc::new(WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Literal(Literal::Integer(1)),
            })),
        );
        assert_eq!(
            cards.mir(card_id).unwrap(),
            Arc::new(Mir {
                entrypoint: ControlFlowGraphId(0),
                cfgs: vec![ControlFlowGraph {
                    parameters: vec![],
                    captured: vec![],
                    output: Type::Number,
                    vars: Vars(vec![Var {
                        ty: Type::Number,
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
