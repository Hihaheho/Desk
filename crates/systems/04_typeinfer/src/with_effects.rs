use crate::{
    ctx::{Ctx, Log},
    ty::{Effect, Type},
};

pub struct WithEffects<T>(pub T, pub Vec<Effect>);

impl WithEffects<Ctx> {
    pub fn recover_effects(self) -> Ctx {
        let effects = self.1.into_iter().map(|effect| Log::Effect(effect));
        self.0.logs.borrow_mut().extend(effects);
        self.0
    }
}

impl WithEffects<(Ctx, Type)> {
    pub fn recover_effects(self) -> (Ctx, Type) {
        let effects = self.1.into_iter().map(|effect| Log::Effect(effect));
        self.0 .0.logs.borrow_mut().extend(effects);
        (self.0 .0, self.0 .1)
    }
}
