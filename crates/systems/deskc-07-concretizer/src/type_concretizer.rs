use mir::ty::{ConcEffect, ConcEffectExpr, ConcType};
use types::{Effect, EffectExpr, Type};

pub struct TypeConcretizer {}

impl TypeConcretizer {
    pub fn gen_conc_type(&mut self, ty: &Type) -> ConcType {
        match ty {
            Type::Number => ConcType::Number,
            Type::String => ConcType::String,
            Type::Product(types) => {
                ConcType::Tuple(types.iter().map(|t| self.gen_conc_type(t)).collect())
            }
            Type::Sum(types) => {
                ConcType::Enum(types.iter().map(|t| self.gen_conc_type(t)).collect())
            }
            Type::Function { parameters, body } => ConcType::Function {
                parameters: parameters.iter().map(|t| self.gen_conc_type(t)).collect(),
                body: Box::new(self.gen_conc_type(body)),
            },
            Type::Vector(ty) => ConcType::Array(Box::new(self.gen_conc_type(ty))),
            Type::Set(ty) => ConcType::Set(Box::new(self.gen_conc_type(ty))),
            Type::Variable(id) => ConcType::Variable(id.clone()),
            Type::ForAll { variable, body } => ConcType::ForAll {
                variable: variable.clone(),
                body: Box::new(self.gen_conc_type(body)),
            },
            Type::Effectful { ty, effects } => ConcType::Effectful {
                ty: Box::new(self.gen_conc_type(ty)),
                effects: self.gen_conc_effect_expr(effects),
            },
            Type::Brand { brand: label, item } | Type::Label { label, item } => ConcType::Label {
                label: label.clone(),
                item: Box::new(self.gen_conc_type(item)),
            },
        }
    }

    pub fn gen_conc_effect(&mut self, Effect { input, output }: &Effect) -> ConcEffect {
        ConcEffect {
            input: self.gen_conc_type(input),
            output: self.gen_conc_type(output),
        }
    }

    pub fn gen_conc_effect_expr(&mut self, expr: &EffectExpr) -> ConcEffectExpr {
        match expr {
            EffectExpr::Effects(effects) => {
                ConcEffectExpr::Effects(effects.iter().map(|e| self.gen_conc_effect(e)).collect())
            }
            EffectExpr::Add(effects) => ConcEffectExpr::Add(
                effects
                    .iter()
                    .map(|e| self.gen_conc_effect_expr(e))
                    .collect(),
            ),
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => ConcEffectExpr::Sub {
                minuend: Box::new(self.gen_conc_effect_expr(minuend)),
                subtrahend: Box::new(self.gen_conc_effect_expr(subtrahend)),
            },
            EffectExpr::Apply {
                function,
                arguments,
            } => ConcEffectExpr::Apply {
                function: Box::new(self.gen_conc_type(function)),
                arguments: arguments.iter().map(|a| self.gen_conc_type(a)).collect(),
            },
        }
    }
}
