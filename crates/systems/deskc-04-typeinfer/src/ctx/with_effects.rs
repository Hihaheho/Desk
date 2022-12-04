use crate::{
    ctx::{Ctx, Log},
    ty::effect_expr::EffectExpr,
};

use super::with_type::WithType;

pub struct WithEffects<T>(pub T, pub EffectExpr);

impl WithEffects<Ctx> {
    pub fn recover_effects(self) -> Ctx {
        self.0.logs.borrow_mut().push(Log::Effect(self.1));
        self.0
    }
}

impl WithEffects<WithType<Ctx>> {
    pub fn recover_effects(self) -> WithType<Ctx> {
        self.0 .0.logs.borrow_mut().push(Log::Effect(self.1));
        self.0 .0.with_type(self.0 .1)
    }
}
