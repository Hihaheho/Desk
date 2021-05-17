use protocol::id::Id;

#[derive(Clone)]
pub struct BasicTypeId(pub Id);
#[derive(Clone)]
pub struct EMonadId(pub Id);
#[derive(Clone)]
pub struct EffectId(pub Id);
#[derive(Clone)]
pub struct TypeVariable(pub u8);

#[derive(Clone)]
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
        effects: std::collections::HashSet<EffectId>,
    },
    Variable(TypeVariable),
    Construct {
        constructor: Box<Type>,
        arguments: Vec<Type>,
    },
    Effect {
        emonad_id: EMonadId,
        effect_id: EffectId,
        argument: Box<Type>,
        output: Box<Type>,
    },
}
