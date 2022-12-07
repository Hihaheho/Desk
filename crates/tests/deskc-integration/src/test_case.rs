use std::collections::HashMap;

use dprocess::value::Value;
use ids::{Entrypoint, FileId};
use serde::{Deserialize, Serialize};
use types::Type;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TestCase {
    pub files: Vec<File>,
    pub assertions: Assertions,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub id: FileId,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Assertions {
    pub runs: Option<Vec<Run>>,
    pub typed: Option<Vec<Typed>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub entrypoint: Entrypoint,
    pub result: RunResult,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RunResult {
    Success(Value),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Typed {
    pub entrypoint: Entrypoint,
    pub typings: HashMap<usize, Type>,
}
