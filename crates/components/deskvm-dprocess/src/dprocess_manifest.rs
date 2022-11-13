use std::sync::Arc;

use crate::{
    effect_handler::EffectHandlers, interpreter_builder::InterpreterBuilder, metas::Metas,
};

#[derive(Debug, Clone)]
pub struct DProcessManifest {
    pub interpreter_builder: Arc<dyn InterpreterBuilder>,
    pub effect_handlers: EffectHandlers,
    pub metas: Metas,
}

impl DProcessManifest {
    pub fn new(
        interpreter_builder: impl InterpreterBuilder + 'static,
        effect_handlers: EffectHandlers,
        metas: Metas,
    ) -> Self {
        Self {
            interpreter_builder: Arc::new(interpreter_builder),
            effect_handlers,
            metas,
        }
    }
}
