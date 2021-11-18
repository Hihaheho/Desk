use types::{Effect, Type};

pub(crate) fn from_hir_type(ty: &hir::ty::Type) -> Type {
    use hir::ty::Type::*;
    match ty {
        Number => Type::Number,
        String => Type::String,
        Trait(types) => todo!(),
        Effectful { ty, effects } => todo!(),
        Infer => todo!(),
        This => todo!(),
        Product(types) => {
            Type::Product(types.into_iter().map(|t| from_hir_type(&t.value)).collect())
        }
        Sum(types) => Type::Sum(types.into_iter().map(|t| from_hir_type(&t.value)).collect()),
        Function { parameter, body } => Type::Function {
            parameter: Box::new(from_hir_type(&parameter.value)),
            body: Box::new(from_hir_type(&body.value)),
        },
        Array(ty) => Type::Array(Box::new(from_hir_type(&ty.value))),
        Set(ty) => Type::Set(Box::new(from_hir_type(&ty.value))),
        Let { definition, body } => todo!(),
        Variable(id) => Type::Variable(*id),
        BoundedVariable { bound, identifier } => todo!(),
    }
}

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
