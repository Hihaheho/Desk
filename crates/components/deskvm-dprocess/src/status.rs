use types::{Effect, Type};

use crate::dprocess::DProcessId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DProcessStatus {
    Running,
    Finished,
    SuspendedWithEffect(Effect),
    Delegated { effect: Effect, to: DProcessId },
    WaitingForMessage(Type),
}
