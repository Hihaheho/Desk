use types::Type;

use crate::{block::ABasicBlock, scope::Scope, stmt::LinkId, var::AVar};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Amir {
    // function parameters
    pub parameters: Vec<Type>,
    // implicit parameters that captured from outer scope.
    pub captured: Vec<Type>,
    pub output: Type,
    // first N items in vars are arguments.
    pub vars: Vec<AVar>,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<ABasicBlock>,
    pub links: Vec<LinkId>,
}

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AmirId(pub usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Amirs {
    pub entrypoint: AmirId,
    pub amirs: Vec<Amir>,
}

impl Amir {
    pub fn get_type(&self) -> Type {
        Type::Function {
            parameters: self.parameters.clone(),
            body: Box::new(self.output.clone()),
        }
    }
}
