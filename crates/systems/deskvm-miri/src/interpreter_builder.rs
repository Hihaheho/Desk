use std::collections::HashMap;

use dprocess::{interpreter::Interpreter, interpreter_builder::InterpreterBuilder};
use mir::mir::Mir;
use thiserror::Error;
use types::Type;

use crate::{eval_mir, value::Value};

#[derive(Error, Debug)]
pub enum MiriBuilderCreationError {
    #[error("Parameter not found {0:?}")]
    ParameterNotFound(Type),
}

#[derive(Debug)]
pub struct MiriBuilder {
    pub mir: Mir,
    pub parameters: HashMap<Type, Value>,
}

impl InterpreterBuilder for MiriBuilder {
    fn build(&self) -> Box<dyn Interpreter> {
        Box::new(eval_mir(self.mir.clone(), self.parameters.clone()))
    }
}
