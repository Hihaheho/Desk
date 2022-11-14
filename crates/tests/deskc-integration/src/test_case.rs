use serde::{Deserialize, Serialize};

use crate::assertion::Assertion;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TestCase {
    pub files: Vec<File>,
    // file name
    pub entrypoint: String,
    pub assertions: Vec<Assertion>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub content: String,
}
