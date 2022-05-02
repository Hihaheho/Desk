use mir::ty::{ConcEffect, ConcEffectExpr, ConcType};
use types::{Effect, EffectExpr, Type};

pub struct TypeConcretizer {}

impl TypeConcretizer {
    pub fn to_conc_type(&mut self, ty: &Type) -> ConcType {
        match ty {
            Type::Number => ConcType::Number,
            Type::String => ConcType::String,
            Type::Product(types) => {
                ConcType::Tuple(types.iter().map(|t| self.to_conc_type(t)).collect())
            }
            Type::Sum(types) => {
                ConcType::Enum(types.iter().map(|t| self.to_conc_type(t)).collect())
            }
            Type::Function { parameters, body } => ConcType::Function {
                parameters: parameters.iter().map(|t| self.to_conc_type(t)).collect(),
                body: Box::new(self.to_conc_type(body)),
            },
            Type::Array(ty) => ConcType::Array(Box::new(self.to_conc_type(ty))),
            Type::Set(ty) => ConcType::Set(Box::new(self.to_conc_type(ty))),
            Type::Variable(id) => ConcType::Variable(id.clone()),
            Type::ForAll { variable, body } => ConcType::ForAll {
                variable: variable.clone(),
                body: Box::new(self.to_conc_type(body)),
            },
            Type::Effectful { ty, effects } => ConcType::Effectful {
                ty: Box::new(self.to_conc_type(ty)),
                effects: self.to_conc_effect_expr(effects),
            },
            Type::Brand { brand: label, item } | Type::Label { label, item } => ConcType::Label {
                label: label.clone(),
                item: Box::new(self.to_conc_type(item)),
            },
        }
    }

    pub fn to_conc_effect(&mut self, Effect { input, output }: &Effect) -> ConcEffect {
        ConcEffect {
            input: self.to_conc_type(input),
            output: self.to_conc_type(output),
        }
    }

    pub fn to_conc_effect_expr(&mut self, expr: &EffectExpr) -> ConcEffectExpr {
        match expr {
            EffectExpr::Effects(effects) => {
                ConcEffectExpr::Effects(effects.iter().map(|e| self.to_conc_effect(e)).collect())
            }
            EffectExpr::Add(effects) => ConcEffectExpr::Add(
                effects
                    .iter()
                    .map(|e| self.to_conc_effect_expr(e))
                    .collect(),
            ),
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => ConcEffectExpr::Sub {
                minuend: Box::new(self.to_conc_effect_expr(minuend)),
                subtrahend: Box::new(self.to_conc_effect_expr(subtrahend)),
            },
            EffectExpr::Apply {
                function,
                arguments,
            } => ConcEffectExpr::Apply {
                function: Box::new(self.to_conc_type(function)),
                arguments: arguments.iter().map(|a| self.to_conc_type(a)).collect(),
            },
        }
    }
}
