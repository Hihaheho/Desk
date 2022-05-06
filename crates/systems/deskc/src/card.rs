mod amir;
mod amir_environment;
mod mir;
mod mir_environment;
mod thir;

use amir::amir;
use amir_environment::amir_environment;
use deskc_amir::{amir::Amirs, environment::AEnvironment};
use deskc_hir::meta::WithMeta;
use deskc_ids::CardId;
use deskc_mir::{environment::Environment, mir::Mirs};
use deskc_thir::TypedHir;
use deskc_types::Type;
use mir::mir;
use mir_environment::mir_environment;
use thir::thir;
use uuid::Uuid;

use crate::query_result::QueryResult;

#[salsa::query_group(CardStorage)]
pub trait CardQueries {
    #[salsa::input]
    fn hir(&self, id: CardId) -> QueryResult<WithMeta<deskc_hir::expr::Expr>>;
    fn thir(&self, id: CardId) -> QueryResult<TypedHir>;
    fn amir(&self, id: CardId) -> QueryResult<Amirs>;
    fn mir(&self, id: CardId) -> QueryResult<Mirs>;
    #[salsa::input]
    fn definition(&self, ty: Type, uuid: Uuid) -> QueryResult<Amirs>;
    #[salsa::input]
    fn latest_definition(&self, id: CardId) -> Uuid;
    fn amir_environment(&self, id: CardId) -> QueryResult<AEnvironment>;
    fn mir_environment(&self, id: CardId) -> QueryResult<Environment>;
}

#[salsa::database(CardStorage)]
#[derive(Default)]
pub struct CardDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for CardDatabase {}
