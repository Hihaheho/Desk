use types::Type;

use crate::region::RegionId;

pub type Id = usize;

pub struct ConcTypeId(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConcEffect {
    pub input: ConcType,
    pub output: ConcType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConcType {
    Number,
    String,
    Product(Vec<Self>),
    Sum(Vec<Self>),
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
        inner: Box<Self>,
    },
    RefMut {
        region: RegionId,
        inner: Box<Self>,
    },
    Label {
        labels: Vec<Label>,
        inner: Box<Self>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label {
	pub ty: Type,
	pub label: usize,
}

impl ConcType {
    pub fn product(mut types: Vec<Self>) -> Self {
        types.sort();
        ConcType::Product(types)
    }
    pub fn sum(mut types: Vec<Self>) -> Self {
        types.sort();
        ConcType::Sum(types)
    }
    pub fn function(mut parameters: Vec<Self>, body: Self) -> Self {
        parameters.sort();
        ConcType::Function {
            parameters,
            body: Box::new(body),
        }
    }
    pub fn effectful(ty: Self, mut effects: Vec<ConcEffect>) -> Self {
        effects.sort();
        ConcType::Effectful {
            ty: Box::new(ty),
            effects,
        }
    }

    /// Returns `true` if the conc type is [`Label`].
    ///
    /// [`Label`]: ConcType::Label
    pub fn is_label(&self) -> bool {
        matches!(self, Self::Label { .. })
    }
}
