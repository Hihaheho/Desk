use ty::Type;

use crate::{status::LinkExit, value::Value};

#[derive(Debug)]
pub enum ExitStatus {
    /// All codes are reduced.
    Returned,
    /// The process is halted with the reason.
    Halted {
        ty: Type,
        reason: Value,
    },
    /// The process is crashed with the error.
    /// any crash equals to any crash in PartialEq.
    Crashed(anyhow::Error),
    HaltedByLink(LinkExit),
}

impl PartialEq for ExitStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ExitStatus::Returned, ExitStatus::Returned) => true,
            (
                ExitStatus::Halted {
                    ty: ty1,
                    reason: reason1,
                },
                ExitStatus::Halted {
                    ty: ty2,
                    reason: reason2,
                },
            ) => ty1 == ty2 && reason1 == reason2,
            (ExitStatus::Crashed(_), ExitStatus::Crashed(_)) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use ty::Type;

    use super::*;

    #[test]
    fn exit_status_equals() {
        assert_eq!(ExitStatus::Returned, ExitStatus::Returned);
        assert_eq!(
            ExitStatus::Halted {
                ty: Type::Real,
                reason: Value::String("a".into())
            },
            ExitStatus::Halted {
                ty: Type::Real,
                reason: Value::String("a".into())
            },
        );
        assert_eq!(
            ExitStatus::Crashed(anyhow::anyhow!("a")),
            ExitStatus::Crashed(anyhow::anyhow!("b")),
        );
    }

    #[test]
    fn exit_status_not_equals() {
        assert_ne!(
            ExitStatus::Returned,
            ExitStatus::Halted {
                ty: Type::Real,
                reason: Value::String("a".into())
            }
        );
        assert_ne!(
            ExitStatus::Returned,
            ExitStatus::Crashed(anyhow::anyhow!(""))
        );
        assert_ne!(
            ExitStatus::Halted {
                ty: Type::Real,
                reason: Value::String("a".into())
            },
            ExitStatus::Crashed(anyhow::anyhow!(""))
        );
    }

    #[test]
    fn halted_not_equals() {
        assert_ne!(
            ExitStatus::Halted {
                ty: Type::Real,
                reason: Value::String("a".into())
            },
            ExitStatus::Halted {
                ty: Type::String,
                reason: Value::String("a".into())
            }
        );
        assert_ne!(
            ExitStatus::Halted {
                ty: Type::Real,
                reason: Value::String("a".into())
            },
            ExitStatus::Halted {
                ty: Type::Real,
                reason: Value::String("b".into())
            }
        );
    }
}
