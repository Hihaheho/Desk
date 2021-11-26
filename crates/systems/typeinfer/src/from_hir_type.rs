use hir::meta::WithMeta;

use crate::{
    ty::{Effect, Type},
    Ctx,
};

pub(crate) fn from_hir_type(ctx: &Ctx, ty: &WithMeta<hir::ty::Type>) -> Type {
    use hir::ty::Type::*;
    match &ty.value {
        Number => Type::Number,
        String => Type::String,
        Trait(types) => todo!(),
        Effectful { ty, effects } => Type::Effectful {
            ty: Box::new(from_hir_type(ctx, ty)),
            effects: effects
                .iter()
                .map(|effect| Effect {
                    input: from_hir_type(ctx, &effect.input),
                    output: from_hir_type(ctx, &effect.output),
                })
                .collect(),
        },
        Infer => Type::Infer(ty.meta.as_ref().expect("infer type should have meta").id),
        This => todo!(),
        Product(types) => {
            Type::Product(types.into_iter().map(|t| from_hir_type(ctx, &t)).collect())
        }
        Sum(types) => Type::Sum(types.into_iter().map(|t| from_hir_type(ctx, &t)).collect()),
        Function { parameter, body } => Type::Function {
            parameter: Box::new(from_hir_type(ctx, &parameter)),
            body: Box::new(from_hir_type(ctx, &body)),
        },
        Array(ty) => Type::Array(Box::new(from_hir_type(ctx, &ty))),
        Set(ty) => Type::Set(Box::new(from_hir_type(ctx, &ty))),
        Let { definition, body } => todo!(),
        Variable(id) => Type::Variable(*id),
        BoundedVariable { bound, identifier } => todo!(),
        Brand { brand, item } => Type::Brand {
            brand: brand.clone(),
            item: Box::new(from_hir_type(ctx, &item)),
        },
        Label { label, item } => Type::Label {
            label: label.clone(),
            item: Box::new(from_hir_type(ctx, &item)),
        },
    }
}
