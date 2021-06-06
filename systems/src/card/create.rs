use editor::card::Card;
use protocol::card_id::CardId;

pub fn create_card() -> Card {
    Card {
        card_id: CardId::new(),
    }
}
