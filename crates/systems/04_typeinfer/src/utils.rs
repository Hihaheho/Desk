use crate::{
    ctx::Ctx,
    ty::{Effect, Type},
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
    let mut sum = types
        .into_iter()
        .map(|ty| match ty {
            Type::Sum(sum) => sum,
            other => vec![other],
        })
        .reduce(|a, b| a.into_iter().chain(b).collect())
        .unwrap_or(vec![]);

    sum.sort();
    sum.dedup();
    if sum.len() == 1 {
        sum.pop().unwrap()
    } else {
        Type::Sum(sum)
    }
}
