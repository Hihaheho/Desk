use types::Effect;

use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpreterOutput {
    Returned(Value),
    Performed { input: Value, effect: Effect },
    Running,
}
