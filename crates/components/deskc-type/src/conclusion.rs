use std::collections::HashMap;

use ids::NodeId;

use crate::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeConclusions {
    pub types: HashMap<NodeId, Type>,
    pub cast_strategies: HashMap<TypeToType, CastStrategy>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeToType {
    pub from: Type,
    pub to: Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CastStrategy(pub HashMap<Type, Type>);

impl TypeConclusions {
    pub fn get_type(&self, id: &NodeId) -> Option<&Type> {
        self.types.get(id)
    }
}
