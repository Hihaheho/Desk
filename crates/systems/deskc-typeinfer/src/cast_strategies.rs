use std::collections::HashMap;

use crate::internal_type::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CastStrategy {
    ProductToProduct(HashMap<Type, Type>),
    SumToSum(HashMap<Type, Type>),
    ProductToInner(Type),
    InnerToSum(Type),
}
