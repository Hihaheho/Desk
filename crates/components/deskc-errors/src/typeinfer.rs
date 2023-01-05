use std::fmt::{Display, Formatter};

use hir::{expr::Expr, meta::Meta};
use thiserror::Error;
use ty::{Effect, Type};

use crate::textual_diagnostics::{Report, TextualDiagnostics};

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

#[derive(Debug, PartialEq, Eq)]
pub enum TypeOrString {
    Type(Type),
    String(String),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TypeError {
    #[error("not applicable {expr:?} {ty:?}")]
    NotApplicable { expr: Box<Expr>, ty: TypeOrString },
    #[error("not subtype {sub:?} {ty:?}")]
    NotSubtype { sub: TypeOrString, ty: TypeOrString },
    #[error("circular existential {id:?} {ty:?}")]
    CircularExistential { id: usize, ty: TypeOrString },
    #[error("not instantiable subtype {ty:?}")]
    NotInstantiableSubtype { ty: TypeOrString },
    #[error("not instantiable supertype {ty:?}")]
    NotInstantiableSupertype { ty: TypeOrString },
    #[error("variable not typed {id}")]
    VariableNotTyped { id: usize },
    #[error("unknown effect handled: {effect:?}")]
    UnknownEffectHandled { effect: Effect },
    #[error("continue out of handle")]
    ContinueOutOfHandle,
    #[error("existential not instansiated {id}")]
    ExistentialNotInstansiated { id: usize },
    #[error("not inferred {id:?}")]
    NotInferred { id: ids::NodeId },
    #[error("ambiguous subtype {sub:?} {ty:?}")]
    AmbiguousSubtype { sub: TypeOrString, ty: TypeOrString },
    #[error("sum subtype {sub_ty:?} has unsufficent elements to supertype {super_ty:?}")]
    SumInsufficentElements {
        sub_ty: Vec<TypeOrString>,
        super_ty: Vec<TypeOrString>,
    },
    #[error("product supertype {super_ty:?} has unsufficent elements to subtype {sub_ty:?}")]
    ProductInsufficentElements {
        sub_ty: Vec<TypeOrString>,
        super_ty: Vec<TypeOrString>,
    },
}

impl From<&ExprTypeError> for TextualDiagnostics {
    fn from(error: &ExprTypeError) -> TextualDiagnostics {
        TextualDiagnostics {
            title: "Typeinfer error".into(),
            reports: vec![Report {
                span: todo!(),
                text: format!("{:?}", error.error),
            }],
        }
    }
}

impl From<Type> for TypeOrString {
    fn from(ty: Type) -> Self {
        Self::Type(ty)
    }
}
