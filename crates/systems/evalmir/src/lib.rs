pub mod value;

use std::any::Any;

use types::Types;

#[derive(Debug, Default)]
pub struct EvalMir {
    pub expr_types: Types,
    pub registers: Vec<Box<dyn Any>>,
}

impl EvalMir {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal() {}

    #[test]
    fn builtin() {}
}
