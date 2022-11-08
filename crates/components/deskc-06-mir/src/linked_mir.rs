use std::collections::HashMap;

use ids::LinkId;

use crate::{mir::Mir, ty::ConcType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedMir {
    mirs: Mir,
    links: HashMap<LinkId<ConcType>, Mir>,
}
