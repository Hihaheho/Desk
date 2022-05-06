use deskc_ids::CardId;
use deskc_mir::mir::Mirs;

use super::{KernelQueries, KernelResult};

pub(super) fn mir(db: &dyn KernelQueries, id: CardId) -> KernelResult<Mirs> {
    todo!()
}
