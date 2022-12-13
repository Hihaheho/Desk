use std::collections::HashMap;

use ids::NodeId;

use crate::Type;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
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
pub enum CastStrategy {
    ProductToProduct(HashMap<Type, Type>),
    SumToSum(HashMap<Type, Type>),
    ProductToInner(Type),
    InnerToSum(Type),
}

impl TypeConclusions {
    pub fn get_type(&self, id: &NodeId) -> Option<&Type> {
        self.types.get(id)
    }
}
