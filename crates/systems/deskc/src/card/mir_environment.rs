use deskc_ids::CardId;
use deskc_mir::environment::Environment;

use super::{CardQueries, QueryResult};

pub(super) fn mir_environment(db: &dyn CardQueries, id: CardId) -> QueryResult<Environment> {
    todo!()
}
