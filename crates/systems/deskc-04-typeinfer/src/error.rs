use crate::ty::{Effect, Id, Type};
use hir::{expr::Expr, meta::Meta};
use textual_diagnostics::{Report, TextualDiagnostics};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct ExprTypeError {
    pub meta: Meta,
    pub error: TypeError,
}

#[derive(Error, Debug, PartialEq)]
pub enum TypeError {
    #[error("not applicable")]
    NotApplicable { expr: Box<Expr>, ty: Type },
    #[error("not subtype")]
    NotSubtype { sub: Type, ty: Type },
    #[error("circular existential")]
    CircularExistential { id: Id, ty: Type },
    #[error("not instantiable subtype")]
    NotInstantiableSubtype { ty: Type },
    #[error("not instantiable supertype")]
    NotInstantiableSupertype { ty: Type },
    #[error("variable not typed {id}")]
    VariableNotTyped { id: Id },
    #[error("unknown effect handled: {effect:?}")]
    UnknownEffectHandled { effect: Effect },
    #[error("continue out of handle")]
    ContinueOutOfHandle,
}

impl From<ExprTypeError> for TextualDiagnostics {
    fn from(error: ExprTypeError) -> TextualDiagnostics {
        TextualDiagnostics {
            title: "Typeinfer error".into(),
            reports: vec![Report {
                span: error.meta.span.unwrap_or(0..0),
                text: format!("{:?}", error.error),
            }],
        }
    }
}
