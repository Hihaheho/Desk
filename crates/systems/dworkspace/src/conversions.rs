use deskc_ast::{meta::WithMeta, ty::Type};
use deskc_ty::Type as DeskcType;

pub fn ast_type_to_type(ty: &WithMeta<Type>) -> DeskcType {
    match &ty.value {
        Type::Labeled { brand, item } => todo!(),
        Type::Real => todo!(),
        Type::Rational => todo!(),
        Type::Integer => todo!(),
        Type::String => todo!(),
        Type::Trait(functions) => todo!(),
        Type::Effectful { ty, effects } => todo!(),
        Type::Infer => todo!(),
        Type::This => todo!(),
        Type::Product(types) => todo!(),
        Type::Sum(types) => todo!(),
        Type::Function(function) => todo!(),
        Type::Vector(ty) => todo!(),
        Type::Map { key, value } => todo!(),
        Type::Let {
            variable,
            definition,
            body,
        } => todo!(),
        Type::Variable(ident) => todo!(),
        Type::Attributed { attr, ty } => todo!(),
        Type::Comment { text, item } => todo!(),
        Type::Forall {
            variable,
            bound,
            body,
        } => todo!(),
        Type::Exists {
            variable,
            bound,
            body,
        } => todo!(),
    }
}
