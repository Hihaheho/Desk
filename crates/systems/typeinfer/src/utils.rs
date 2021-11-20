use crate::ty::{Effect, Type};

pub(crate) fn with_effects(ty: Type, effects: Vec<Effect>) -> Type {
    if effects.is_empty() {
        ty
    } else {
        Type::Effectful {
            ty: Box::new(ty),
            effects,
        }
    }
}
