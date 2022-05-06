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
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct CardId(pub Uuid);

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct FileId(pub Uuid);