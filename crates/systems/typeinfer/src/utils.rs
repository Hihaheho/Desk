use crate::ty::{Effect, Type};

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
        Brand { brand, item } => Type::Brand {
            brand: brand.clone(),
            item: Box::new(from_hir_type(&item.value)),
        },
        Label { label, item } => Type::Label {
            label: label.clone(),
            item: Box::new(from_hir_type(&item.value)),
        },
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

pub(crate) fn into_type(ty: &Type) -> types::Type {
    match ty {
        Type::Number => types::Type::Number,
        Type::String => types::Type::String,
        Type::Product(types) => {
            types::Type::product(types.into_iter().map(|t| into_type(&t)).collect())
        }
        Type::Sum(types) => types::Type::sum(types.into_iter().map(|t| into_type(&t)).collect()),
        Type::Function { parameter, body } => {
            if let Type::Function { .. } = **body {
                match into_type(body) {
                    types::Type::Function {
                        mut parameters,
                        body,
                    } => {
                        parameters.insert(0, into_type(parameter));
                        types::Type::function(parameters, *body)
                    }
                    _ => panic!(),
                }
            } else {
                types::Type::function(vec![into_type(parameter)], into_type(body))
            }
        }
        Type::Array(ty) => types::Type::Array(Box::new(into_type(ty))),
        Type::Set(ty) => types::Type::Set(Box::new(into_type(ty))),
        Type::Variable(id) => types::Type::Variable(*id),
        Type::ForAll { variable, body } => types::Type::ForAll {
            variable: *variable,
            body: Box::new(into_type(body)),
        },
        Type::Existential(id) => panic!("should be substituted before"),
        Type::Effectful { ty, effects } => types::Type::effectful(
            into_type(&**ty),
            effects
                .into_iter()
                .map(|Effect { input, output }| types::Effect {
                    input: into_type(input),
                    output: into_type(output),
                })
                .collect(),
        ),
        Type::Brand { brand, item } => types::Type::Brand {
            brand: brand.clone(),
            item: Box::new(into_type(item)),
        },
        Type::Label { label, item } => types::Type::Label {
            label: label.clone(),
            item: Box::new(into_type(item)),
        },
    }
}
