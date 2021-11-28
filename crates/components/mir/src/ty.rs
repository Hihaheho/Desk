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

impl ConcType {
    pub fn needs_cast_to(&self, other: &Self) -> bool {
        match (self, other) {
            (x, y) if x == y => false,
            (
                ConcType::Effectful { ty, effects },
                ConcType::Effectful {
                    ty: ty2,
                    effects: effects2,
                },
            ) if !ty.needs_cast_to(ty2) => {
                effects
                    .iter()
                    .zip(effects2.iter())
                    .any(|(effect, effect2)| {
                        effect.input.needs_cast_to(&effect2.input)
                            || effect.output.needs_cast_to(&effect2.output)
                    })
            }
            (ConcType::Label { label: _, item }, x) if !item.needs_cast_to(x) => false,
            (x, ConcType::Label { label: _, item }) if !item.needs_cast_to(x) => false,
            (ConcType::Enum(types), ConcType::Enum(types2)) => types
                .iter()
                .zip(types2.iter())
                .any(|(x, y)| x.needs_cast_to(y)),
            (ConcType::Tuple(types), ConcType::Tuple(types2)) => types
                .iter()
                .zip(types2.iter())
                .any(|(x, y)| x.needs_cast_to(y)),
            (x, ConcType::Effectful { ty, effects: _ }) if !x.needs_cast_to(ty) => false,
            _ => true,
        }
    }
}
