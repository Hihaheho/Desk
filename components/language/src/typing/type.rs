use std::collections::HashSet;

use protocol::id::Id;

pub struct BasicTypeId(Id);
pub struct EMonadId(Id);
pub struct EffectId(Id);
pub struct TypeVariable(u8);

pub enum Type {
    Basic {
        basic_type_id: BasicTypeId,
        parameters: Vec<Type>,
    },
    // curried function
    Function {
        parameter: Box<Type>,
        output: Box<Type>,
    },
    EMonad {
        emonad_id: EMonadId,
        parameters: Vec<Type>,
        item: Box<Type>,
        effects: HashSet<Effect>,
    },
    Variable(TypeVariable),
    Construct {
        constructor: Box<Type>,
        arguments: Vec<Type>,
    },
}

pub struct Effect {
    pub emonad_id: EMonadId,
    pub type_id: EffectId,
    pub argument: Box<Type>,
    pub output: Box<Type>,
}
