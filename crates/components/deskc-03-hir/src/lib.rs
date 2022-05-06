use expr::Expr;
use ids::CardId;
use meta::WithMeta;

pub mod expr;
pub mod meta;
pub mod ty;

pub struct Hir {
    pub entrypoint: Option<WithMeta<Expr>>,
    pub cards: Vec<(CardId, WithMeta<Expr>)>,
}
