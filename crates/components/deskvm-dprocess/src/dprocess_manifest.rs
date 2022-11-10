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
