pub mod stack;
pub mod value;

use std::cell::RefCell;

use stack::Stack;
use types::{Type, Types};
use value::Value;

#[derive(Debug, Clone, Default)]
pub struct EvalMir {
    pub expr_types: Types,
    pub stacks: RefCell<Vec<Stack>>,
}

impl EvalMir {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal() {
        let evalmir = EvalMir::default();
        // assert_eq!(
        //     evalmir.eval(),
        //     Value::Int(1)
        // );
    }

    #[test]
    fn builtin() {
        // let mir = parse(r#"<\'number, 'number -> @added 'number> 1, 2"#);
        // let evalmir = EvalMir {
        //     expr_types: infer(&hir),
        //     ..Default::default()
        // };
        // assert_eq!(evalmir.eval(&hir), Value::Int(3));
    }
}
