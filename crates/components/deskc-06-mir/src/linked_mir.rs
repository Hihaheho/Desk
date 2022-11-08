use std::collections::HashMap;

use conc_types::ConcType;
use ids::LinkId;

use crate::mir::Mir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedMir {
    mirs: Mir,
    links: HashMap<LinkId<ConcType>, Mir>,
}
