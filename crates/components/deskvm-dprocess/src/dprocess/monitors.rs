use std::sync::Arc;

use types::Type;

use crate::{
    status::{DProcessStatus, LinkExit},
    value::Value,
};

use super::{DProcess, DProcessId};

impl DProcess {
    pub fn add_monitor(&self, monitor: &DProcess) {
        match &*self.read_status() {
            DProcessStatus::Returned(value) => {
                monitor.notify_down(DownMessage {
                    from: self.id.clone(),
                    payload: DownPayload::Returned(value.clone()),
                });
            }
            DProcessStatus::Crashed(_) => {
                monitor.notify_down(DownMessage {
                    from: self.id.clone(),
                    payload: DownPayload::Crashed,
                });
            }
            DProcessStatus::Halted { ty, reason } => {
                monitor.notify_down(DownMessage {
                    from: self.id.clone(),
                    payload: DownPayload::Halted {
                        ty: ty.clone(),
                        reason: reason.clone(),
                    },
                });
            }
            _ => {
                let mut monitors = self.lock_monitors();
                monitors.insert(monitor.id.clone());
            }
        }
    }

    pub fn remove_monitor(&self, monitor: &DProcessId) {
        let mut monitors = self.lock_monitors();
        monitors.remove(monitor);
    }

    pub fn notify_down(&self, _message: DownMessage) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct DownMessage {
    pub from: DProcessId,
    pub payload: DownPayload,
}

#[derive(Debug, Clone)]
pub enum DownPayload {
    Returned(Arc<Value>),
    Crashed,
    Halted { ty: Arc<Type>, reason: Arc<Value> },
    NotFound,
    LinkExit(LinkExit),
}
