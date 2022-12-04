use std::sync::Arc;

use ast::span::WithSpan;
use codebase::code::Code;
use hir::meta::WithMeta;
use ids::CardId;
use mir::mir::Mir;
use thir::TypedHir;

use crate::{parse_source_code, query_result::QueryResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirResult {
    pub hir: WithMeta<hir::expr::Expr>,
    pub next_id: usize,
}

#[salsa::query_group(CardStorage)]
pub trait CardQueries {
    #[salsa::input]
    fn code(&self, card_id: CardId) -> Code;
    fn ast(&self, id: CardId) -> QueryResult<WithSpan<ast::expr::Expr>>;
    fn typeinfer(&self, id: CardId) -> QueryResult<typeinfer::ctx::Ctx>;
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

fn ast(db: &dyn CardQueries, id: CardId) -> QueryResult<WithSpan<ast::expr::Expr>> {
    let code = db.code(id);
    match code {
        Code::SourceCode { syntax, source } => {
            let ast = parse_source_code(&syntax, &source)?;
            Ok(Arc::new(ast))
        }
        Code::Ast(ast) => Ok(ast),
    }
}

fn hir(db: &dyn CardQueries, id: CardId) -> QueryResult<HirResult> {
    let ast = db.ast(id)?;
    let (genhir, hir) = hirgen::gen_hir(&ast)?;
    Ok(Arc::new(HirResult {
        hir,
        next_id: genhir.next_id(),
    }))
}

fn typeinfer(db: &dyn CardQueries, id: CardId) -> QueryResult<typeinfer::ctx::Ctx> {
    let hir_result = db.hir(id)?;
    let (ctx, _ty) = typeinfer::synth(hir_result.next_id, &hir_result.hir)?;
    Ok(Arc::new(ctx))
}

fn thir(db: &dyn CardQueries, id: CardId) -> QueryResult<TypedHir> {
    let hir_result = db.hir(id.clone())?;
    let ctx = db.typeinfer(id)?;
    let thir = thirgen::gen_typed_hir(ctx.next_id(), ctx.get_types(), &hir_result.hir);
    Ok(Arc::new(thir))
}

fn mir(db: &dyn CardQueries, id: CardId) -> QueryResult<Mir> {
    let thir = db.thir(id)?;
    let mir = mirgen::gen_mir(&thir)?;
    Ok(Arc::new(mir))
}
