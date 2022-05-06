use deskc_ids::CardId;
use deskc_mir::environment::Environment;

use super::{KernelQueries, KernelResult};

pub(super) fn execution_context(db: &dyn KernelQueries, id: CardId) -> KernelResult<Environment> {
    todo!()
}
