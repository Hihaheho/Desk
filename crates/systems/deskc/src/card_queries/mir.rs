use deskc_ids::CardId;
use deskc_mir::mir::Mirs;

use super::{CardQueries, QueryResult};

pub(super) fn mir(db: &dyn CardQueries, id: CardId) -> QueryResult<Mirs> {
    todo!()
}
