use std::fmt::Debug;

use crate::interpreter::Interpreter;

pub trait InterpreterBuilder: Debug {
    fn build(&self) -> Box<dyn Interpreter>;
}
