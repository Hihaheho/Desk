use std::collections::HashMap;

use ids::LinkId;

use crate::{mir::Mirs, ty::ConcType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment {
    mirs: Mirs,
    links: HashMap<LinkId<ConcType>, Mirs>,
}
