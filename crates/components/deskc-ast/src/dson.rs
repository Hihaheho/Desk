use dson::{Dson, Real};
use thiserror::Error;

use crate::{
    expr::{Expr, MapElem},
    meta::WithMeta,
    ty::Type,
};

#[derive(Error, Debug)]
pub enum ExprToDsonError {
    #[error("unexpected do")]
    UnexpectedDo,
    #[error("unexpected let")]
    UnexpectedLet,
    #[error("unexpected perform")]
    UnexpectedPerform,
    #[error("unexpected continue")]
    UnexpectedContinue,
    #[error("unexpected handle")]
    UnexpectedHandle,
    #[error("unexpected apply")]
    UnexpectedApply,
    #[error("unexpected match")]
    UnexpectedMatch,
    #[error("unexpected hole")]
    UnexpectedHole,
    #[error("unexpected function")]
    UnexpectedFunction,
    #[error("unexpected brand")]
    UnexpectedBrand,
    #[error("unexpected new type")]
    UnexpectedNewType,
    #[error("unexpected card")]
    UnexpectedCard,
    #[error("unexpected effectful")]
    UnexpectedEffectful,
    #[error("unexpected infer")]
    UnexpectedInfer,
    #[error("unexpected function type")]
    UnexpectedFunctionTy,
    #[error("unexpected forall")]
    UnexpectedForall,
    #[error("unexpected exists")]
    UnexpectedExists,
}

impl TryFrom<WithMeta<Expr>> for Dson {
    type Error = ExprToDsonError;

    fn try_from(expr: WithMeta<Expr>) -> Result<Self, Self::Error> {
        match expr.value {
            Expr::Literal(literal) => Ok(Dson::Literal(match literal {
                crate::expr::Literal::String(string) => dson::Literal::String(string),
                crate::expr::Literal::Integer(integer) => dson::Literal::Integer(integer),
                crate::expr::Literal::Rational(a, b) => dson::Literal::Rational(a, b),
                crate::expr::Literal::Real(float) => dson::Literal::Real(Real(float)),
            })),
            Expr::Product(exprs) => Ok(Dson::Product(
                exprs
                    .into_iter()
                    .map(Dson::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            Expr::Typed { ty, item } => Ok(Dson::Typed {
                ty: ty.try_into()?,
                expr: Box::new(Dson::try_from(*item)?),
            }),
            Expr::Vector(exprs) => Ok(Dson::Vector(
                exprs
                    .into_iter()
                    .map(Dson::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            Expr::Map(elems) => Ok(Dson::Map(
                elems
                    .into_iter()
                    .map(|elem| {
                        let MapElem { key, value } = elem.value;
                        Ok(dson::MapElem {
                            key: Dson::try_from(key)?,
                            value: Dson::try_from(value)?,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            Expr::Attributed { attr, item } => Ok(Dson::Attributed {
                attr: Box::new(attr),
                expr: Box::new(Dson::try_from(*item)?),
            }),
            Expr::Label { label, item } => Ok(Dson::Labeled {
                label,
                expr: Box::new(Dson::try_from(*item)?),
            }),
            Expr::Do { .. } => Err(ExprToDsonError::UnexpectedDo),
            Expr::Let { .. } => Err(ExprToDsonError::UnexpectedLet),
            Expr::Perform { .. } => Err(ExprToDsonError::UnexpectedPerform),
            Expr::Continue { .. } => Err(ExprToDsonError::UnexpectedContinue),
            Expr::Handle { .. } => Err(ExprToDsonError::UnexpectedHandle),
            Expr::Apply { .. } => Err(ExprToDsonError::UnexpectedApply),
            Expr::Match { .. } => Err(ExprToDsonError::UnexpectedMatch),
            Expr::Hole => Err(ExprToDsonError::UnexpectedHole),
            Expr::Function { .. } => Err(ExprToDsonError::UnexpectedFunction),
            Expr::DeclareBrand { .. } => Err(ExprToDsonError::UnexpectedBrand),
            Expr::NewType { .. } => Err(ExprToDsonError::UnexpectedNewType),
            Expr::Card { .. } => Err(ExprToDsonError::UnexpectedCard),
        }
    }
}

impl TryFrom<WithMeta<Type>> for dson::Type {
    type Error = ExprToDsonError;

    fn try_from(ty: WithMeta<Type>) -> Result<Self, Self::Error> {
        match ty.value {
            Type::Labeled { brand, item } => Ok(dson::Type::Brand {
                brand,
                item: Box::new(dson::Type::try_from(*item)?),
            }),
            Type::Real => Ok(dson::Type::Real),
            Type::Rational => Ok(dson::Type::Rational),
            Type::Integer => Ok(dson::Type::Integer),
            Type::String => Ok(dson::Type::String),
            Type::Product(types) => Ok(dson::Type::Product(
                types
                    .into_iter()
                    .map(dson::Type::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            Type::Sum(types) => Ok(dson::Type::Sum(
                types
                    .into_iter()
                    .map(dson::Type::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            Type::Vector(ty) => Ok(dson::Type::Vector(Box::new(dson::Type::try_from(*ty)?))),
            Type::Map { key, value } => Ok(dson::Type::Map {
                key: Box::new(dson::Type::try_from(*key)?),
                value: Box::new(dson::Type::try_from(*value)?),
            }),
            Type::Let {
                variable,
                definition,
                body,
            } => Ok(dson::Type::Let {
                variable,
                definition: Box::new(dson::Type::try_from(*definition)?),
                body: Box::new(dson::Type::try_from(*body)?),
            }),
            Type::Variable(ident) => Ok(dson::Type::Variable(ident)),
            Type::Attributed { attr, ty } => Ok(dson::Type::Attributed {
                attr: Box::new(attr),
                ty: Box::new(dson::Type::try_from(*ty)?),
            }),
            Type::Effectful { .. } => Err(ExprToDsonError::UnexpectedEffectful),
            Type::Infer => Err(ExprToDsonError::UnexpectedInfer),
            Type::Function(_) => Err(ExprToDsonError::UnexpectedFunctionTy),
            Type::Forall { .. } => Err(ExprToDsonError::UnexpectedForall),
            Type::Exists { .. } => Err(ExprToDsonError::UnexpectedExists),
        }
    }
}
