mod card;
mod query_result;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ast::{
        expr::{Expr, Literal},
        span::WithSpan,
    };
    use codebase::code::{Code, SyntaxKind};
    use conc_types::ConcType;
    use ids::{CardId, NodeId};
    use mir::{
        mir::{BasicBlock, ControlFlowGraph, Mir, Var},
        stmt::Stmt,
        Const, ControlFlowGraphId, Scope, ScopeId, StmtBind, Terminator, VarId, Vars,
    };

    use crate::card::{CardQueries, CardsCompiler};

    #[test]
    fn compiles_source_code() {
        let mut cards = CardsCompiler::default();
        let card_id = CardId::new();
        cards.set_code(
            card_id.clone(),
            Code::SourceCode {
                syntax: SyntaxKind::Hacker,
                source: Arc::new("* 1, 2".into()),
            },
        );
        assert_eq!(
            cards.mir(card_id).unwrap(),
            Arc::new(Mir {
                entrypoint: ControlFlowGraphId(0),
                cfgs: vec![ControlFlowGraph {
                    parameters: vec![],
                    captured: vec![],
                    output: ConcType::Number,
                    vars: Vars(vec![
                        Var {
                            ty: ConcType::Number,
                            scope: ScopeId(0)
                        },
                        Var {
                            ty: ConcType::Number,
                            scope: ScopeId(0)
                        },
                        Var {
                            ty: ConcType::Tuple(vec![ConcType::Number, ConcType::Number]),
                            scope: ScopeId(0)
                        }
                    ]),
                    scopes: vec![Scope { super_scope: None }],
                    blocks: vec![BasicBlock {
                        stmts: vec![
                            StmtBind {
                                var: VarId(0),
                                stmt: Stmt::Const(Const::Int(1))
                            },
                            StmtBind {
                                var: VarId(1),
                                stmt: Stmt::Const(Const::Int(2))
                            },
                            StmtBind {
                                var: VarId(2),
                                stmt: Stmt::Tuple(vec![VarId(0), VarId(1)])
                            }
                        ],
                        terminator: Terminator::Return(VarId(2))
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
                value: Expr::Product(vec![
                    WithSpan {
                        id: NodeId::new(),
                        span: 0..0,
                        value: Expr::Literal(Literal::Integer(1)),
                    },
                    WithSpan {
                        id: NodeId::new(),
                        span: 0..0,
                        value: Expr::Literal(Literal::Integer(2)),
                    },
                ]),
            })),
        );
        assert_eq!(
            cards.mir(card_id).unwrap(),
            Arc::new(Mir {
                entrypoint: ControlFlowGraphId(0),
                cfgs: vec![ControlFlowGraph {
                    parameters: vec![],
                    captured: vec![],
                    output: ConcType::Number,
                    vars: Vars(vec![
                        Var {
                            ty: ConcType::Number,
                            scope: ScopeId(0)
                        },
                        Var {
                            ty: ConcType::Number,
                            scope: ScopeId(0)
                        },
                        Var {
                            ty: ConcType::Tuple(vec![ConcType::Number, ConcType::Number]),
                            scope: ScopeId(0)
                        }
                    ]),
                    scopes: vec![Scope { super_scope: None }],
                    blocks: vec![BasicBlock {
                        stmts: vec![
                            StmtBind {
                                var: VarId(0),
                                stmt: Stmt::Const(Const::Int(1))
                            },
                            StmtBind {
                                var: VarId(1),
                                stmt: Stmt::Const(Const::Int(2))
                            },
                            StmtBind {
                                var: VarId(2),
                                stmt: Stmt::Tuple(vec![VarId(0), VarId(1)])
                            }
                        ],
                        terminator: Terminator::Return(VarId(2))
                    }],
                    links: vec![]
                }]
            })
        );
    }
}
