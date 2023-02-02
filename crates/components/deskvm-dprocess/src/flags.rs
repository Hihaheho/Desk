#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Inspired on Erlang's ones.
pub struct DProcessFlags {
    priority: Priority,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Priority {
    /// The process might be not scheduled.
    Min,
    /// Low priority than default.
    Low,
    /// Normal priority.
    #[default]
    Default,
    /// High priority than default.
    High,
    /// The process should be scheduled always.
    Max,
    /// A priority for internal use. It has same priority as `Max` or has higher priority than `Max`.
    InternalMax,
}
