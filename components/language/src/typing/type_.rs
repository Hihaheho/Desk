use crate::util::set::Set;

// Ord is required for sort and dedup type list.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Type {
    Unit,
    /// Type label
    Label(String),
    Bool,
    String,
    /// integer, rational number, or float
    Number,
    /// An ordered collection of some type.
    /// implementation might be linked list.
    Array(Box<Type>),
    /// aka. tuple or record.
    /// Not Vec<Type>!
    Product(Set<Type>),
    /// aka. Enum in Rust
    Sum(Set<Type>),
    Variable(String),
    /// curried function
    Function(Arrow),
    Trait(Vec<Arrow>),
    Effect {
        class: Class,
        effect: Arrow,
    },
    /// Marker for showing
    Effectful {
        item: Box<Type>,
        class: Class,
        handled: bool,
    },
}

/// argument => output
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Arrow {
    pub argument: Box<Type>,
    pub output: Box<Type>,
}

/// Trait, EMonad,  
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Class {
    pub arrows: Set<Arrow>,
}
