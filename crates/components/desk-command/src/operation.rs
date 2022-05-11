#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalOperation {
    UpdateCommandInput(String),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TerminalOperations(Vec<TerminalOperation>);

impl TerminalOperations {
    pub fn iter(&self) -> impl Iterator<Item = &TerminalOperation> {
        self.0.iter()
    }
}

impl<T: Iterator<Item = TerminalOperation>> From<T> for TerminalOperations {
    fn from(from: T) -> Self {
        Self(from.collect::<Vec<_>>())
    }
}
