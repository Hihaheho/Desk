use types::Type;

use crate::value::Value;

#[derive(Debug)]
pub enum ExitStatus {
    /// All codes are reduced.
    Finished,
    /// The process is halted with the reason.
    Halted { ty: Type, reason: Value },
    /// The process is crashed with the error.
    /// any crash equals to any crash in PartialEq.
    Crashed(anyhow::Error),
}

impl PartialEq for ExitStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ExitStatus::Finished, ExitStatus::Finished) => true,
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
    use types::Type;

    use super::*;

    #[test]
    fn exit_status_equals() {
        assert_eq!(ExitStatus::Finished, ExitStatus::Finished);
        assert_eq!(
            ExitStatus::Halted {
                ty: Type::Number,
                reason: Value::String("a".into())
            },
            ExitStatus::Halted {
                ty: Type::Number,
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
            ExitStatus::Finished,
            ExitStatus::Halted {
                ty: Type::Number,
                reason: Value::String("a".into())
            }
        );
        assert_ne!(
            ExitStatus::Finished,
            ExitStatus::Crashed(anyhow::anyhow!(""))
        );
        assert_ne!(
            ExitStatus::Halted {
                ty: Type::Number,
                reason: Value::String("a".into())
            },
            ExitStatus::Crashed(anyhow::anyhow!(""))
        );
    }

    #[test]
    fn halted_not_equals() {
        assert_ne!(
            ExitStatus::Halted {
                ty: Type::Number,
                reason: Value::String("a".into())
            },
            ExitStatus::Halted {
                ty: Type::String,
                reason: Value::String("a".into())
            }
        );
        assert_ne!(
            ExitStatus::Halted {
                ty: Type::Number,
                reason: Value::String("a".into())
            },
            ExitStatus::Halted {
                ty: Type::Number,
                reason: Value::String("b".into())
            }
        );
    }
}
