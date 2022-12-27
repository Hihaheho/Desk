use std::{collections::HashMap, sync::Arc};

use deskc_type::{conclusion::TypeConclusions, Type};
use dprocess::{interpreter::Interpreter, interpreter_builder::InterpreterBuilder};
use mir::mir::Mir;
use thiserror::Error;

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
    pub type_conclusion: Arc<TypeConclusions>,
}

impl InterpreterBuilder for MiriBuilder {
    fn build(&self) -> Box<dyn Interpreter> {
        Box::new(eval_mir(
            self.mir.clone(),
            self.parameters.clone(),
            self.type_conclusion.clone(),
        ))
    }
}
