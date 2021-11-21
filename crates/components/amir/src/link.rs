use types::Type;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LinkId(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub struct Link {
    pub ty: Type,
}
