use crate::interpreter_builder::InterpreterBuilder;

use super::DProcess;

impl DProcess {
	/// Replaces the current interpreter with new one.
    pub fn reset(&self, interpreter_builder: Box<dyn InterpreterBuilder>) {
        todo!()
    }
}
