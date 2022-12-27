mod cmp;
mod helpers;
mod int;

use deskc_macros::ty;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use deskc_type::Type;
use once_cell::sync::Lazy;

use crate::value::{OperatorOutput, Value};

pub static OPERATORS: Lazy<HashMap<Type, Operator>> = Lazy::new(|| {
    Operator::iter()
        .map(|operator| (operator.ty(), operator))
        .collect()
});

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum Operator {
    IntAdd,
    IntSub,
    IntMul,
    IntDiv,
    Rem,
    RealEq,
    RealCmp,
}

impl Operator {
    pub fn ty(&self) -> Type {
        match self {
            Operator::IntAdd => {
                ty!(r#"\ *<@"l" 'integer, @"r" 'integer> -> @"sum" 'integer"#)
            }
            Operator::IntSub => {
                ty!(r#"\ *<@"l" 'integer, @"r" 'integer> -> @"diff" 'integer"#)
            }
            Operator::IntMul => {
                ty!(r#"\ *<@"l" 'integer, @"r" 'integer> -> @"prod" 'integer"#)
            }
            Operator::IntDiv => ty!(r#"
				\ *<@"l" 'integer, @"r" 'integer> -> ! {
			    	@"division by zero" 'integer ~> @"quot" 'integer
				} @"quot" 'integer
				"#),
            Operator::Rem => ty!(r#"
				\ *<@"l" 'integer, @"r" 'integer> -> ! {
					@"division by zero" 'integer ~> *<@"quot" 'integer, @"rem" 'integer>
				} *<@"quot" 'integer, @"rem" 'integer>
				"#),
            Operator::RealEq => {
                ty!(r#"\ *<@"l" 'real, @"r" 'real> -> +<@"equal" *<>, @"unequal" *<>>"#)
            }
            Operator::RealCmp => {
                ty!(
                    r#"\ *<@"l" 'real, @"r" 'real> -> +<@"less" *<>, @"equal" *<>, @"greater" *<>>"#
                )
            }
        }
    }

    pub fn call(&self, value: &Value) -> OperatorOutput {
        match self {
            Operator::IntAdd => int::add(value),
            Operator::IntSub => int::sub(value),
            Operator::IntMul => int::mul(value),
            Operator::IntDiv => int::div(value),
            Operator::Rem => todo!(),
            Operator::RealEq => cmp::real_eq(value),
            Operator::RealCmp => cmp::real_cmp(value),
        }
    }
}
