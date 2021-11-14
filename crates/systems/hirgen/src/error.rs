use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum HirGenError {}
