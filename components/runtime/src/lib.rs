pub mod code_generator;
pub mod definition;
pub mod vm;

use language::{semantic::ir::IR, typing::type_::Type};

/// A struct for a computed value with its type and encoding.
pub struct ComputedValue {
    pub type_: Type,
    pub encoded_value: EncodedValue,
}

pub enum EncodedValue {
    Function(IR),
    I32(i32),
    F32(f32),
    String(String),
}
