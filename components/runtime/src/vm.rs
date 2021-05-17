use crate::ComputedValue;

pub trait VM {
    type Code;

    fn run(code: &Self::Code) -> ComputedValue;
}
