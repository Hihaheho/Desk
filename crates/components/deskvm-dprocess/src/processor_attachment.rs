use crate::processor::ProcessorName;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProcessorAttachment {
    Attached(ProcessorName),
    Detached,
}

impl Default for ProcessorAttachment {
    fn default() -> Self {
        Self::Detached
    }
}
