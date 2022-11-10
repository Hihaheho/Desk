#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProcessingKind {
    /// Current processing mainly uses GPU.
    GPU,
    /// Current processing mainly uses CPU.
    CPU,
    /// Current processing mainly uses IO.
    IO,
}
