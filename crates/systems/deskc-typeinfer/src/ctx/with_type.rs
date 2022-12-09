use crate::internal_type::Type;

use super::Ctx;

pub struct WithType<Ctx>(pub Ctx, pub Type);

impl WithType<Ctx> {
    pub fn ctx_do<E>(self, f: impl FnOnce(&Ctx) -> Result<Ctx, E>) -> Result<WithType<Ctx>, E> {
        let WithType(ctx, ty) = self;
        Ok(WithType(f(&ctx)?, ty))
    }
}
