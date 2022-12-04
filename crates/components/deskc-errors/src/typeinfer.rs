use std::fmt::{Display, Formatter};

use hir::{expr::Expr, meta::Meta};
use thiserror::Error;
use types::{Type, Effect};

use crate::textual_diagnostics::{TextualDiagnostics, Report};

#[derive(Debug, PartialEq, Eq)]
pub struct ExprTypeError {
    pub meta: Meta,
    pub error: TypeError,
}

impl Display for ExprTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.meta, self.error)
    }
}

impl std::error::Error for ExprTypeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TypeError {
    #[error("not applicable")]
    NotApplicable { expr: Box<Expr>, ty: Type },
    #[error("not subtype")]
    NotSubtype { sub: Type, ty: Type },
    #[error("circular existential")]
    CircularExistential { id: usize, ty: Type },
    #[error("not instantiable subtype")]
    NotInstantiableSubtype { ty: Type },
    #[error("not instantiable supertype")]
    NotInstantiableSupertype { ty: Type },
    #[error("variable not typed {id}")]
    VariableNotTyped { id: usize },
    #[error("unknown effect handled: {effect:?}")]
    UnknownEffectHandled { effect: Effect },
    #[error("continue out of handle")]
    ContinueOutOfHandle,
}

impl From<&ExprTypeError> for TextualDiagnostics {
    fn from(error: &ExprTypeError) -> TextualDiagnostics {
        TextualDiagnostics {
            title: "Typeinfer error".into(),
            reports: vec![Report {
                span: error.meta.span.clone().unwrap_or(0..0),
                text: format!("{:?}", error.error),
            }],
        }
    }
}