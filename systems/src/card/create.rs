use bevy_math::Vec2;
use editor::card::Card;
use protocol::card_id::CardId;

pub fn create_card(position: Vec2) -> Card {
    Card {
        card_id: CardId::new(),
        position,
    }
}
