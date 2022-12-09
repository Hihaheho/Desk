use anyhow::Result;
use std::{collections::HashMap, time::Duration};
use ty::Effect;

use crate::{interpreter_output::InterpreterOutput, processing_kind::ProcessingKind, value::Value};

pub trait Interpreter: std::fmt::Debug {
    /// Interpret the code within the given duration.
    ///
    /// Implementation should not exceed the given duration.
    /// Implementation can return an output earlier even if it remains codes to run.
    fn reduce(&mut self, target_duration: &Duration) -> Result<InterpreterOutput>;

    /// Receive an output of performing effect.
    ///
    /// It must do nothing other than receiving.
    fn effect_output(&mut self, value: Value);

    /// Returns the current processing kind.
    fn current_processing_kind(&self) -> Result<SchedulingHint<ProcessingKind>> {
        Ok(SchedulingHint::NotSupported)
    }

    /// Returns estimated time to finish the current processing kind.
    ///
    /// The parameter contains effects that have estimated time to handle.
    fn estimated_current_processing_kind(
        &self,
        _effect_handler_hint: &EffectHandlerHint,
    ) -> Result<SchedulingHint<FinishEstimation>> {
        Ok(SchedulingHint::NotSupported)
    }

    /// Returns estimated time for the next target effect in the target effects.
    ///
    /// Should return Duration::MAX if there is no effect.
    /// Scheduler may use this to know when the interpreter performs blocking effects.
    fn estimate_next_effect(
        &self,
        _target_effects: &[Effect],
    ) -> Result<SchedulingHint<NextEffectEstimation>> {
        Ok(SchedulingHint::NotSupported)
    }

    /// Returns estimated time to finish.
    ///
    /// Should return Duration::MAX if one of more effects may be performed.
    /// The parameter contains effects that have estimated time to handle.
    fn estimate_finish(&self) -> Result<SchedulingHint<FinishEstimation>> {
        Ok(SchedulingHint::NotSupported)
    }

    /// Returns effects that may be performed.
    ///
    /// It must cover all effects that may be performed.
    fn possible_effects(&self) -> Result<SchedulingHint<PossibleEffects>> {
        Ok(SchedulingHint::NotSupported)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A hint to tell an interpreter how long VM handles the effects.
///
/// Only includes effects that can be estimated.
pub struct EffectHandlerHint {
    pub effects: HashMap<Effect, Duration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulingHint<T> {
    /// Don't use this in an implementation.
    NotSupported,
    ImpossibleOrTimeConsuming,
    Provided(T),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextEffectEstimation {
    Effect {
        effect: Effect,
        duration: Duration,
    },
    /// An interpreter can return this value if it's not possible to estimate the duration.
    NoEffects,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinishEstimation {
    Duration(Duration),
    MayPerformEffects,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PossibleEffects {
    Effects(Vec<Effect>),
}
