use std::collections::HashMap;

use ids::LinkId;
use types::Type;

use crate::amir::Amir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedAmir {
    mirs: Amir,
    links: HashMap<LinkId<Type>, Amir>,
}
