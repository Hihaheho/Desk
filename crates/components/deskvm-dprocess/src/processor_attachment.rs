use crate::processor::ProcessorName;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProcessorAttachment {
    Attached(ProcessorName),
    #[default]
    Detached,
}
