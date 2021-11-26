use types::Type;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LinkId(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub struct ALink<T=Type> {
    pub ty: T,
}
