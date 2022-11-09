use std::{collections::HashMap, sync::Arc};

use conc_types::ConcType;
use ids::LinkId;

use crate::mir::Mir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedMir {
    mirs: Arc<Mir>,
    links: HashMap<LinkId<ConcType>, Arc<Mir>>,
}
