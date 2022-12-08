use crate::interpreter::Interpreter;

pub trait InterpreterBuilder: std::fmt::Debug {
    fn build(&self) -> Box<dyn Interpreter>;
}
