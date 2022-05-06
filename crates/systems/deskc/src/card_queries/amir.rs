use deskc_amir::amir::Amirs;
use deskc_ids::CardId;

use super::{CardQueries, QueryResult};

pub(super) fn amir(db: &dyn CardQueries, id: CardId) -> QueryResult<Amirs> {
    todo!()
}
