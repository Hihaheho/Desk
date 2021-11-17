use hir::expr::Expr;
use thiserror::Error;
use types::{Id, Type};

#[derive(Error, Debug, PartialEq)]
pub enum TypeError {
    #[error("not applicable")]
    NotApplicable { expr: Expr, ty: Type },
    #[error("not subtype")]
    NotSubtype { sub: Type, ty: Type },
    #[error("circular existential")]
    CircularExistential { id: Id, ty: Type },
    #[error("not instantiable")]
    NotInstantiable { ty: Type },
    #[error("variable not typed {id}")]
    VariableNotTyped { id: Id },
}
