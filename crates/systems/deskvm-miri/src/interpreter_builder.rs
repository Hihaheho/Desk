use dprocess::{interpreter::Interpreter, interpreter_builder::InterpreterBuilder};
use mir::mir::Mir;

use crate::eval_mir;

#[derive(Debug)]
pub struct MiriBuilder(pub Mir);

impl InterpreterBuilder for MiriBuilder {
    fn build(&self) -> Box<dyn Interpreter> {
        Box::new(eval_mir(self.0.clone()))
    }
}
