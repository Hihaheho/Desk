use uuid::Uuid;

#[derive(Clone, Copy, Debug, Hash)]
pub struct CardId(Uuid);

impl ToString for CardId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl CardId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

pub struct Card {
    pub id: CardId,
}

impl Card {
    pub fn new() -> Self {
        Self { id: CardId::new() }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

/// A struct for a computed value with its type and encoding.
pub struct Computed<Node>(pub Node);
