use hir::meta::Meta;
use thiserror::Error;
use ty::Type;

use crate::textual_diagnostics::{Report, TextualDiagnostics};

#[derive(Debug, Clone, Error)]
pub enum GenMirError {
    #[error("invalid function call {ty:?} with {arguments:?}")]
    InvalidFunctionCall {
        expr: Meta,
        ty: Type,
        arguments: Vec<Meta>,
    },
    #[error("type not found {for_expr:?}")]
    TypeNotFound { for_expr: Meta },
    #[error("function inferred as non-function {for_expr:?}")]
    FunctionInferredAsNonFunction { for_expr: hir::meta::Meta },
    #[error("effectful inferred as non-effectful {for_expr:?}")]
    EffectfulInferredAsNonEffectful { for_expr: hir::meta::Meta },
}

impl From<&GenMirError> for TextualDiagnostics {
    fn from(error: &GenMirError) -> TextualDiagnostics {
        match error {
            GenMirError::InvalidFunctionCall {
                expr,
                ty: _,
                arguments: _,
            } => TextualDiagnostics {
                title: "MIR generation error".into(),
                reports: vec![Report {
                    span: expr.span.clone().unwrap_or(0..0),
                    text: format!("{}", error),
                }],
            },
            GenMirError::TypeNotFound { for_expr: meta } => TextualDiagnostics {
                title: "MIR generation error".into(),
                reports: vec![Report {
                    span: meta.span.clone().unwrap_or(0..0),
                    text: format!("{}", error),
                }],
            },
            GenMirError::FunctionInferredAsNonFunction { for_expr: meta } => TextualDiagnostics {
                title: "MIR generation error".into(),
                reports: vec![Report {
                    span: meta.span.clone().unwrap_or(0..0),
                    text: format!("{}", error),
                }],
            },
            GenMirError::EffectfulInferredAsNonEffectful { for_expr: meta } => TextualDiagnostics {
                title: "MIR generation error".into(),
                reports: vec![Report {
                    span: meta.span.clone().unwrap_or(0..0),
                    text: format!("{}", error),
                }],
            },
        }
    }
}
