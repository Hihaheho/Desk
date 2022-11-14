use dson::Dson;
use thir::{Expr, Literal, TypedHir};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HirToJsonError {
    #[error("{0} not allowed")]
    NotAllowed(String),
}

pub fn thir_to_dson(thir: &TypedHir) -> Result<Dson, HirToJsonError> {
    let dson = match &thir.expr {
        Expr::Literal(Literal::Int(value)) => Dson::Literal(dson::Literal::Int(*value)),
        Expr::Literal(Literal::Rational(a, b)) => Dson::Literal(dson::Literal::Rational(*a, *b)),
        Expr::Literal(Literal::Float(value)) => Dson::Literal(dson::Literal::Float(*value)),
        Expr::Literal(Literal::String(value)) => {
            Dson::Literal(dson::Literal::String(value.clone()))
        }
        Expr::Product(values) => Dson::Product(
            values
                .iter()
                .map(thir_to_dson)
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Expr::Vector(values) => Dson::Array(
            values
                .iter()
                .map(thir_to_dson)
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Expr::Set(values) => Dson::Set(
            values
                .iter()
                .map(thir_to_dson)
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Expr::Let { .. } => return Err(HirToJsonError::NotAllowed("let".into())),
        Expr::Perform { .. } => return Err(HirToJsonError::NotAllowed("perform".into())),
        Expr::Handle { .. } => return Err(HirToJsonError::NotAllowed("handle".into())),
        Expr::Apply { .. } => return Err(HirToJsonError::NotAllowed("apply".into())),
        Expr::Match { .. } => return Err(HirToJsonError::NotAllowed("match".into())),
        Expr::Function { .. } => return Err(HirToJsonError::NotAllowed("function".into())),
        Expr::Op { .. } => return Err(HirToJsonError::NotAllowed("op".into())),
        Expr::Label { label, item: expr } => Dson::Labeled {
            label: label.clone(),
            expr: Box::new(thir_to_dson(expr)?),
        },
    };
    Ok(dson)
}

#[cfg(test)]
mod tests {}
