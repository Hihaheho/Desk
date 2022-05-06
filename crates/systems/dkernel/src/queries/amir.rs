use deskc_amir::amir::Amirs;
use deskc_ids::CardId;

use super::{KernelQueries, KernelResult};

pub(super) fn amir(db: &dyn KernelQueries, id: CardId) -> KernelResult<Amirs> {
    todo!()
}
