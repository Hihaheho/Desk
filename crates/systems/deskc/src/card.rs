use std::sync::Arc;

use ast::span::WithSpan;
use codebase::code::Code;
use hir::meta::WithMeta;
use ids::{Entrypoint, FileId};
use mir::mir::Mir;
use thir::TypedHir;

use crate::{
    error::DeskcError, hir_result::CardsResult, parse_source_code, query_result::QueryResult,
};

#[salsa::query_group(CardStorage)]
pub trait DeskcQueries {
    #[salsa::input]
    fn code(&self, id: FileId) -> Code;
    fn ast(&self, id: FileId) -> QueryResult<WithSpan<ast::expr::Expr>>;
    fn cards(&self, id: FileId) -> QueryResult<CardsResult>;
    fn hir(&self, entrypoint: Entrypoint) -> QueryResult<WithMeta<hir::expr::Expr>>;
    fn typeinfer(&self, entrypoint: Entrypoint) -> QueryResult<typeinfer::ctx::Ctx>;
    fn thir(&self, entrypoint: Entrypoint) -> QueryResult<TypedHir>;
    fn mir(&self, entrypoint: Entrypoint) -> QueryResult<Mir>;
}

#[salsa::database(CardStorage)]
#[derive(Default)]
pub struct DeskCompiler {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DeskCompiler {}

fn ast(db: &dyn DeskcQueries, id: FileId) -> QueryResult<WithSpan<ast::expr::Expr>> {
    let code = db.code(id);
    match code {
        Code::SourceCode { syntax, source } => {
            let ast = parse_source_code(&syntax, &source)?;
            Ok(Arc::new(ast))
        }
        Code::Ast(ast) => Ok(ast),
    }
}

fn cards(db: &dyn DeskcQueries, id: FileId) -> QueryResult<CardsResult> {
    let ast = db.ast(id)?;
    let (genhir, hir) = hirgen::gen_cards(&ast)?;
    Ok(Arc::new(CardsResult {
        cards: hir,
        next_id: genhir.next_id(),
    }))
}

fn hir(db: &dyn DeskcQueries, entrypoint: Entrypoint) -> QueryResult<WithMeta<hir::expr::Expr>> {
    let cards_result = db.cards(entrypoint.file_id().clone())?;
    let hir = match entrypoint {
        Entrypoint::Card { file_id, card_id } => cards_result
            .cards
            .cards
            .iter()
            .find(|card| card.id == card_id)
            .map(|card| Ok(card.hir.clone()))
            .unwrap_or_else(|| Err(DeskcError::CardNotFound { card_id, file_id }))?,
        Entrypoint::File(_) => cards_result.cards.file.clone(),
    };
    Ok(Arc::new(hir))
}

fn typeinfer(db: &dyn DeskcQueries, entrypoint: Entrypoint) -> QueryResult<typeinfer::ctx::Ctx> {
    let cards_result = db.cards(entrypoint.file_id().clone())?;
    let hir = db.hir(entrypoint)?;
    let (ctx, _ty) = typeinfer::synth(cards_result.next_id, &hir)?;
    Ok(Arc::new(ctx))
}

fn thir(db: &dyn DeskcQueries, entrypoint: Entrypoint) -> QueryResult<TypedHir> {
    let hir = db.hir(entrypoint.clone())?;
    let ctx = db.typeinfer(entrypoint)?;
    let thir = thirgen::gen_typed_hir(ctx.next_id(), ctx.get_types(), &hir);
    Ok(Arc::new(thir))
}

fn mir(db: &dyn DeskcQueries, entrypoint: Entrypoint) -> QueryResult<Mir> {
    let thir = db.thir(entrypoint)?;
    let mir = mirgen::gen_mir(&thir)?;
    Ok(Arc::new(mir))
}
