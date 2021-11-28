use mir::ty::{ConcEffect, ConcType};
use types::{Effect, Type};

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
                effects: effects
                    .iter()
                    .map(|effect| self.to_conc_effect(effect))
                    .collect(),
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
}
