use std::sync::Arc;

use ty::{Effect, Type};

use crate::{dprocess::DProcessId, value::Value};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// A status of process.
///
/// It's cheap to clone.
pub enum DProcessStatus {
    #[default]
    Running,
    Deferred {
        effect: Arc<Effect>,
        input: Arc<Value>,
    },
    WaitingForMessage(Arc<Type>),
    Returned(Arc<Value>),
    Halted {
        ty: Arc<Type>,
        reason: Arc<Value>,
    },
    Crashed(CrashError),
    HaltedByLink(LinkExit),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkExit {
    Halted {
        dprocess_id: DProcessId,
        ty: Arc<Type>,
        reason: Arc<Value>,
    },
    Crashed {
        dprocess_id: DProcessId,
        error: CrashError,
    },
    NotFound(DProcessId),
}

#[derive(Debug, Clone)]
pub struct CrashError(pub Arc<anyhow::Error>);

impl PartialEq for CrashError {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

impl Eq for CrashError {}

impl From<anyhow::Error> for CrashError {
    fn from(error: anyhow::Error) -> Self {
        Self(Arc::new(error))
    }
}
