use std::fmt::Display;

use dson::Dson;
use serde::{de, ser};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),
    #[error("expected a bool but got {got:?}")]
    ExpectedBool { got: Dson },
    #[error("expected a unit but got {got:?}")]
    ExpectedUnit { got: Dson },
    #[error("expected a product but got {got:?}")]
    ExpectedProduct { got: Dson },
    #[error("expected a map but got {got:?}")]
    ExpectedMap { got: Dson },
    #[error("expected a label but got {got:?}")]
    ExpectedLabel { got: Dson },
    #[error("expected a string literal but got {got:?}")]
    ExpectedString { got: Dson },
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
