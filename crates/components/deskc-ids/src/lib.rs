use uuid::Uuid;

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
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

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct LinkId<Type> {
    pub ty: Type,
    pub name: LinkName,
}

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord, Default)]
pub struct CardId(pub Uuid);

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct FileId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NodeId(pub Uuid);

impl NodeId {
    pub fn new() -> Self {
        NodeId(Uuid::new_v4())
    }
}

impl FileId {
    pub fn new() -> Self {
        FileId(Uuid::new_v4())
    }
}

impl CardId {
    pub fn new() -> Self {
        CardId(Uuid::new_v4())
    }
}
