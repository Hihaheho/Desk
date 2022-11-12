use std::sync::Arc;

use ast::span::WithSpan;
use codebase::code::Code;
use hir::meta::WithMeta;
use ids::CardId;
use mir::mir::Mir;
use thir::TypedHir;
use tokens::Tokens;

use crate::query_result::QueryResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirResult {
    hir: WithMeta<hir::expr::Expr>,
    next_id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
// AST might be inputed as a code, so it doesn't need to output tokens.
pub enum TokensOrAst {
    Token(QueryResult<Tokens>),
    Ast(Arc<WithSpan<ast::expr::Expr>>),
}

#[salsa::query_group(CardStorage)]
pub trait CardQueries {
    #[salsa::input]
    fn code(&self, card_id: CardId) -> Code;
    fn tokens_or_ast(&self, id: CardId) -> TokensOrAst;
    fn ast(&self, id: CardId) -> QueryResult<WithSpan<ast::expr::Expr>>;
    fn hir(&self, id: CardId) -> QueryResult<HirResult>;
    fn thir(&self, id: CardId) -> QueryResult<TypedHir>;
    fn mir(&self, id: CardId) -> QueryResult<Mir>;
}

#[salsa::database(CardStorage)]
#[derive(Default)]
pub struct CardsCompiler {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for CardsCompiler {}

fn tokens_or_ast(db: &dyn CardQueries, id: CardId) -> TokensOrAst {
    let code = db.code(id);
    match code {
        Code::SourceCode { syntax: _, source } => {
            let result = || {
                let tokens = lexer::scan(&source)?;
                Ok(Arc::new(tokens))
            };
            TokensOrAst::Token(result())
        }
        Code::Ast(ast) => TokensOrAst::Ast(ast),
    }
}

fn ast(db: &dyn CardQueries, id: CardId) -> QueryResult<WithSpan<ast::expr::Expr>> {
    match db.tokens_or_ast(id) {
        TokensOrAst::Token(tokens) => {
            let tokens = tokens?;
            let ast = parser::parse(tokens.as_ref().clone())?;
            Ok(Arc::new(ast))
        }
        TokensOrAst::Ast(ast) => Ok(ast),
    }
}

fn hir(db: &dyn CardQueries, id: CardId) -> QueryResult<HirResult> {
    let ast = db.ast(id)?;
    let (genhir, hir) = hirgen::gen_hir(&ast).unwrap();
    Ok(Arc::new(HirResult {
        hir,
        next_id: genhir.next_id(),
    }))
}

fn thir(db: &dyn CardQueries, id: CardId) -> QueryResult<TypedHir> {
    let hir_result = db.hir(id)?;
    let (ctx, _ty) = typeinfer::synth(hir_result.next_id, &hir_result.hir)?;
    let thir = thirgen::gen_typed_hir(ctx.next_id(), ctx.get_types(), &hir_result.hir);
    Ok(Arc::new(thir))
}

fn mir(db: &dyn CardQueries, id: CardId) -> QueryResult<Mir> {
    let thir = db.thir(id)?;
    let mir = mirgen::gen_mir(&thir).unwrap();
    Ok(Arc::new(mir))
}
