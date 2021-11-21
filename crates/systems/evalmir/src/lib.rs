pub mod stack;
pub mod value;

use std::cell::RefCell;

use stack::Stack;
use types::Types;

#[derive(Debug, Clone, Default)]
pub struct EvalAmir {
    pub expr_types: Types,
    pub stacks: RefCell<Vec<Stack>>,
}

impl EvalAmir {}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn literal() {
        // let evalamir = EvalAmir::default();
        // assert_eq!(
        //     evalamir.eval(),
        //     Value::Int(1)
        // );
    }

    #[test]
    fn builtin() {
        // let amir = parse(r#"<\'number, 'number -> @added 'number> 1, 2"#);
        // let evalamir = EvalAmir {
        //     expr_types: infer(&hir),
        //     ..Default::default()
        // };
        // assert_eq!(evalamir.eval(&hir), Value::Int(3));
    }
}
