use crate::{
    ctx::{Ctx, Log},
    ty::{effect_expr::EffectExpr, Type},
};

pub struct WithEffects<T>(pub T, pub EffectExpr);

impl WithEffects<Ctx> {
    pub fn recover_effects(self) -> Ctx {
        self.0.logs.borrow_mut().push(Log::Effect(self.1));
        self.0
    }
}

impl WithEffects<(Ctx, Type)> {
    pub fn recover_effects(self) -> (Ctx, Type) {
        self.0 .0.logs.borrow_mut().push(Log::Effect(self.1));
        (self.0 .0, self.0 .1)
    }
}
