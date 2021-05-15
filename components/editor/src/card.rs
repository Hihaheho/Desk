use bevy_math::Vec2;
use protocol::card_id::CardId;

pub struct Card {
    pub card_id: CardId,
    pub position: Vec2,
    // pub widget: Widget,
}
