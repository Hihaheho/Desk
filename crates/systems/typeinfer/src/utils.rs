use crate::{
    ty::{Effect, Type},
    Ctx,
};

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

// TODO: use subtyping before concat or push the type.
pub(crate) fn sum_all(_ctx: &Ctx, types: Vec<Type>) -> Type {
    types
        .into_iter()
        .reduce(|a, b| match (a, b) {
            (Type::Sum(a), Type::Sum(b)) => Type::Sum(a.into_iter().chain(b).collect()),
            (Type::Sum(mut a), b) => {
                a.push(b);
                Type::Sum(a)
            }
            (a, Type::Sum(mut b)) => {
                b.push(a);
                Type::Sum(b)
            }
            (a, b) => Type::Sum(vec![a, b]),
        })
        .unwrap_or(Type::Sum(vec![]))
}
