use card::Card;

pub struct Desk {
    cards: Vec<Card>,
}

impl Desk {
    pub fn new() -> Self {
        Self { cards: vec![] }
    }
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }
}
