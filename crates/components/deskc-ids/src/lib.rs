use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
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

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub struct FileId(pub Uuid);

#[derive(
    Clone, Copy, Debug, PartialEq, Hash, Eq, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub struct CardId(pub Uuid);

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
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

impl FileId {
    pub fn new() -> Self {
        FileId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Entrypoint {
    Card { file_id: FileId, card_id: CardId },
    File(FileId),
}

impl Entrypoint {
    pub fn file_id(&self) -> &FileId {
        match self {
            Entrypoint::Card { file_id, .. } => file_id,
            Entrypoint::File(file_id) => file_id,
        }
    }
}
