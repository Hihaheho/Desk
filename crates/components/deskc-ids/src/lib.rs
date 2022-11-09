use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum LinkName {
    None,
    Version(Uuid),
    Card(Uuid),
}

impl Default for LinkName {
    fn default() -> Self {
        LinkName::None
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct LinkId<Type> {
    pub ty: Type,
    pub name: LinkName,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct CardId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct NodeId(pub Uuid);

impl NodeId {
    pub fn new() -> Self {
        NodeId(Uuid::new_v4())
    }
}

impl CardId {
    pub fn new() -> Self {
        CardId(Uuid::new_v4())
    }
}
