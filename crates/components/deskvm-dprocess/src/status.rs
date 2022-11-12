use std::sync::Arc;

use anyhow::Error;
use types::{Effect, Type};

use crate::{dprocess::DProcessId, value::Value};

#[derive(Debug, Clone)]
pub enum DProcessStatus {
    Running,
    SuspendedWithEffect(Effect),
    WaitingForMessage(Type),
    Returned(Arc<Value>),
    Halted { ty: Arc<Type>, reason: Arc<Value> },
    Crashed(Arc<Error>),
    HaltedByLink(LinkExit),
}

impl Default for DProcessStatus {
    fn default() -> Self {
        Self::Running
    }
}

#[derive(Debug, Clone)]
pub enum LinkExit {
    Halted {
        dprocess_id: DProcessId,
        ty: Arc<Type>,
        reason: Arc<Value>,
    },
    Crashed {
        dprocess_id: DProcessId,
        error: Arc<Error>,
    },
    NotFound(DProcessId),
}
