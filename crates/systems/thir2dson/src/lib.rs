use dson::Dson;
use thir::{Expr, Literal, TypedHir};

pub enum HirToJsonError {
    NotAllowed(String),
}

pub fn thir_to_json(thir: &TypedHir) -> Result<Dson, HirToJsonError> {
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
                .map(|v| thir_to_json(v))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Expr::Array(values) => Dson::Array(
            values
                .iter()
                .map(|v| thir_to_json(v))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Expr::Set(values) => Dson::Set(
            values
                .iter()
                .map(|v| thir_to_json(v))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Expr::Let { .. } => Err(HirToJsonError::NotAllowed("let".into()))?,
        Expr::Perform { .. } => Err(HirToJsonError::NotAllowed("perform".into()))?,
        Expr::Handle { .. } => Err(HirToJsonError::NotAllowed("handle".into()))?,
        Expr::Apply { .. } => Err(HirToJsonError::NotAllowed("apply".into()))?,
        Expr::Match { .. } => Err(HirToJsonError::NotAllowed("match".into()))?,
        Expr::Function { .. } => Err(HirToJsonError::NotAllowed("function".into()))?,
        Expr::Reference => Err(HirToJsonError::NotAllowed("ref".into()))?,
        Expr::Op { .. } => Err(HirToJsonError::NotAllowed("op".into()))?,
    };
    Ok(dson)
}
