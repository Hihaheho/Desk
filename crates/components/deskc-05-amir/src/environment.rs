use std::collections::HashMap;

use ids::LinkId;
use types::Type;

use crate::amir::Amirs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AEnvironment {
    mirs: Amirs,
    links: HashMap<LinkId<Type>, Amirs>,
}
