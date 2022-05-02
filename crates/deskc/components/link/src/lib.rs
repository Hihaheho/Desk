use uuid::Uuid;

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum LinkName {
    Version(Uuid),
    Card(Uuid),
}
