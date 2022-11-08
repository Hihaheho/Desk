use std::sync::Arc;

use amir::amir::Amir;
use ast::span::WithSpan;
use hir::meta::WithMeta;
use ids::CardId;
use mir::mir::Mir;
use thir::TypedHir;

use crate::query_result::QueryResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirResult {
    hir: WithMeta<hir::expr::Expr>,
    next_id: usize,
}

#[salsa::query_group(CardStorage)]
pub trait CardQueries {
    #[salsa::input]
    fn ast(&self, id: CardId) -> WithSpan<ast::expr::Expr>;
    fn hir(&self, id: CardId) -> QueryResult<HirResult>;
    fn thir(&self, id: CardId) -> QueryResult<TypedHir>;
    fn amir(&self, id: CardId) -> QueryResult<Amir>;
    fn mir(&self, id: CardId) -> QueryResult<Mir>;
}

#[salsa::database(CardStorage)]
#[derive(Default)]
pub struct CardDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for CardDatabase {}

pub(super) fn hir(db: &dyn CardQueries, id: CardId) -> QueryResult<HirResult> {
    let ast = db.ast(id);
    let (genhir, hir) = hirgen::gen_hir(&ast).unwrap();
    Ok(Arc::new(HirResult {
        hir,
        next_id: genhir.next_id(),
    }))
}

pub(super) fn thir(db: &dyn CardQueries, id: CardId) -> QueryResult<TypedHir> {
    let hir_result = db.hir(id)?;
    let (ctx, _ty) = typeinfer::synth(hir_result.next_id, &hir_result.hir)?;
    let thir = thirgen::gen_typed_hir(ctx.next_id(), ctx.get_types(), &hir_result.hir);
    Ok(Arc::new(thir))
}

pub(super) fn amir(db: &dyn CardQueries, id: CardId) -> QueryResult<Amir> {
    let thir = db.thir(id)?;
    let amir = amirgen::gen_abstract_mir(&thir).unwrap();
    Ok(Arc::new(amir))
}

pub(super) fn mir(db: &dyn CardQueries, id: CardId) -> QueryResult<Mir> {
    let amir = db.amir(id)?;
    let mir = concretizer::concretize(&amir);
    Ok(Arc::new(mir))
}
