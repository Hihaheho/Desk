use crate::{
    ty::{Effect, Type},
    Ctx,
};

pub(crate) fn into_type(ctx: &Ctx, ty: &Type) -> types::Type {
    match ty {
        Type::Number => types::Type::Number,
        Type::String => types::Type::String,
        Type::Product(types) => {
            types::Type::product(types.into_iter().map(|t| into_type(ctx, &t)).collect())
        }
        Type::Sum(types) => {
            types::Type::sum(types.into_iter().map(|t| into_type(ctx, &t)).collect())
        }
        Type::Function { parameter, body } => {
            if let Type::Function { .. } = **body {
                match into_type(ctx, body) {
                    types::Type::Function {
                        mut parameters,
                        body,
                    } => {
                        parameters.insert(0, into_type(ctx, parameter));
                        types::Type::function(parameters, *body)
                    }
                    _ => panic!(),
                }
            } else {
                types::Type::function(vec![into_type(ctx, parameter)], into_type(ctx, body))
            }
        }
        Type::Array(ty) => types::Type::Array(Box::new(into_type(ctx, ty))),
        Type::Set(ty) => types::Type::Set(Box::new(into_type(ctx, ty))),
        Type::Variable(id) => types::Type::Variable(*id),
        Type::ForAll { variable, body } => types::Type::ForAll {
            variable: *variable,
            body: Box::new(into_type(ctx, body)),
        },
        Type::Effectful { ty, effects } => types::Type::effectful(
            into_type(ctx, &**ty),
            effects
                .into_iter()
                .map(|Effect { input, output }| types::Effect {
                    input: into_type(ctx, input),
                    output: into_type(ctx, output),
                })
                .collect(),
        ),
        Type::Brand { brand, item } => types::Type::Brand {
            brand: brand.clone(),
            item: Box::new(into_type(ctx, item)),
        },
        Type::Label { label, item } => types::Type::Label {
            label: label.clone(),
            item: Box::new(into_type(ctx, item)),
        },
        Type::Existential(id) => into_type(
            ctx,
            ctx.types
                .borrow()
                .get(id)
                .expect(&format!("should be instansiated: {}", id)),
        ),
        Type::Infer(id) => into_type(ctx, ctx.types.borrow().get(id).expect("should be inferred")),
    }
}
