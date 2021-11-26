use crate::region::RegionId;

pub type Id = usize;

pub struct ConcTypeId(pub usize);

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConcEffect {
    pub input: ConcType,
    pub output: ConcType,
}

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConcType {
    Number,
    String,
    Tuple(Vec<Self>),
    Enum(Vec<Self>),
    Function {
        parameters: Vec<Self>,
        body: Box<Self>,
    },
    Array(Box<Self>),
    Set(Box<Self>),
    Variable(Id),
    ForAll {
        variable: Id,
        body: Box<Self>,
    },
    Effectful {
        ty: Box<Self>,
        effects: Vec<ConcEffect>,
    },
    Ref {
        region: RegionId,
        item: Box<Self>,
    },
    RefMut {
        region: RegionId,
        item: Box<Self>,
    },
    Label {
        // Just for distinguish types
        label: String,
        item: Box<Self>,
    },
}
