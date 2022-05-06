use deskc_ids::CardId;
use deskc_thir::TypedHir;

use super::{KernelQueries, KernelResult};

pub(super) fn thir(db: &dyn KernelQueries, id: CardId) -> KernelResult<TypedHir> {
    todo!()
}
