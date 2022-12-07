use hir::Cards;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardsResult {
    pub cards: Cards,
    pub next_id: usize,
}
