use crate::util::set::Set;

// Ord is required for sort and dedup type list.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Type {
    Unit,
    /// Type label
    Label(String),
    String,
    /// integer, rational, or float
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
    Trait(Trait),
    Effect {
        class: Trait,
        effect: Arrow,
    },
    /// Marker for showing the calculation contains effects
    Effectful {
        item: Box<Type>,
        class: Trait,
        handled: bool,
    },
}

/// argument -> output
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Arrow {
    pub input: Box<Type>,
    pub output: Box<Type>,
}

/// Trait, EMonad,  
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Trait(pub Set<Arrow>);

impl Type {
    pub fn product(vec: Vec<Type>) -> Self {
        Self::Product(Set::new(vec))
    }

    pub fn sum(vec: Vec<Type>) -> Self {
        Self::Sum(Set::new(vec))
    }

    /// Returns simplified type if type is verbose.
    ///
    /// - Removes product or sum and extracts its item if length is 1.
    /// - Returns a unit if length is 0.
    pub fn remove_verbose_composite_type(&self) -> &Self {
        use Type::*;
        let set = match self {
            Product(set) => set,
            Sum(set) => set,
            _ => {
                return self;
            }
        };
        match set.len() {
            0 => &Type::Unit,
            1 => set.iter().next().unwrap(),
            _ => self,
        }
    }

    pub fn is_subtype_of(&self, other: &Self) -> bool {
        use Type::*;
        let me = self.remove_verbose_composite_type();
        let you = other.remove_verbose_composite_type();
        if me == you {
            return true;
        }

        match (me, you) {
            (Product(me), you) => {
                if me.contains(you, |me, you| me.is_subtype_of(you)) {
                    return true;
                }
            }
            (me, Sum(you)) => {
                if you.contains(me, |you, me| me.is_subtype_of(you)) {
                    return true;
                }
            }
            _ => {}
        };
        match (me, you) {
            (Label(me), Label(you)) => me == you,
            (Product(me), Product(you)) => me.is_superset_of(you, |me, you| me.is_subtype_of(you)),
            (Sum(me), Sum(you)) => you.is_superset_of(me, |you, me| me.is_subtype_of(you)),
            (Array(me), Array(you)) => me.is_subtype_of(you),
            (Function(me), Function(you)) => me.is_subtype_of(you),
            (Trait(me), Trait(you)) => me.is_subtype_of(you),
            // Currently, I haven't considered any relation between Effects and Effectfuls other than equality
            _ => false,
        }
    }
}

impl Arrow {
    pub fn new(input: Type, output: Type) -> Self {
        Self {
            input: Box::new(input),
            output: Box::new(output),
        }
    }

    pub fn is_subtype_of(&self, other: &Self) -> bool {
        self.output.is_subtype_of(other.output.as_ref())
            && other.input.is_subtype_of(self.input.as_ref())
    }
}

impl Trait {
    pub fn new(vec: Vec<Arrow>) -> Self {
        Self(Set::new(vec))
    }

    pub fn is_subtype_of(&self, other: &Self) -> bool {
        self.0
            .is_superset_of(&other.0, |me, you| me.is_subtype_of(you))
    }
}

#[cfg(test)]
mod test_arrow;
#[cfg(test)]
mod test_trait;
#[cfg(test)]
mod test_type;
