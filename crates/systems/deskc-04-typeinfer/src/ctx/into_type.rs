use crate::{
    ty::{effect_expr::EffectExpr, Type},
    Ctx,
};

use super::Id;

impl Ctx {
    pub(crate) fn gen_type(&self, ty: &Type) -> types::Type {
        match ty {
            Type::Number => types::Type::Number,
            Type::String => types::Type::String,
            Type::Product(types) => {
                types::Type::product(types.iter().map(|t| self.gen_type(t)).collect())
            }
            Type::Sum(types) => types::Type::sum(types.iter().map(|t| self.gen_type(t)).collect()),
            Type::Function { parameter, body } => {
                types::Type::function(self.gen_type(parameter), self.gen_type(body))
            }
            Type::Vector(ty) => types::Type::Vector(Box::new(self.gen_type(ty))),
            Type::Map { key, value } => types::Type::Map {
                key: Box::new(self.gen_type(key)),
                value: Box::new(self.gen_type(value)),
            },
            Type::Variable(id) => types::Type::Variable(self.get_ident_of(*id)),
            Type::ForAll {
                variable,
                bound,
                body,
            } => types::Type::ForAll {
                variable: self.get_ident_of(*variable),
                bound: bound.as_ref().map(|bound| Box::new(self.gen_type(&bound))),
                body: Box::new(self.gen_type(body)),
            },
            Type::Effectful { ty, effects } => types::Type::Effectful {
                ty: Box::new(self.gen_type(ty)),
                effects: self.gen_effect_expr(effects),
            },
            Type::Brand { brand, item } => types::Type::Brand {
                brand: brand.clone(),
                item: Box::new(self.gen_type(item)),
            },
            Type::Label { label, item } => types::Type::Label {
                label: label.clone(),
                item: Box::new(self.gen_type(item)),
            },
            Type::Existential(id) => self.gen_type(
                self.types
                    .borrow()
                    .get(id)
                    .unwrap_or_else(|| panic!("should be instansiated: {}", id)),
            ),
            Type::Infer(id) => {
                self.gen_type(self.ir_types.borrow().get(id).expect("should be inferred"))
            }
        }
    }

    pub(crate) fn gen_effect_expr(&self, expr: &EffectExpr) -> types::EffectExpr {
        match expr {
            EffectExpr::Effects(effects) => types::EffectExpr::Effects(
                effects
                    .iter()
                    .map(|e| types::Effect {
                        input: self.gen_type(&e.input),
                        output: self.gen_type(&e.output),
                    })
                    .collect(),
            ),
            EffectExpr::Add(effects) => {
                types::EffectExpr::Add(effects.iter().map(|e| self.gen_effect_expr(e)).collect())
            }
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => types::EffectExpr::Sub {
                minuend: Box::new(self.gen_effect_expr(minuend)),
                subtrahend: Box::new(self.gen_effect_expr(subtrahend)),
            },
            EffectExpr::Apply {
                function,
                arguments,
            } => types::EffectExpr::Apply {
                function: Box::new(self.gen_type(function)),
                arguments: arguments.iter().map(|a| self.gen_type(a)).collect(),
            },
        }
    }

    fn get_ident_of(&self, id: Id) -> String {
        self.variables_idents
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| {
                let mut id_str = id.to_string();
                while self.variables_ids.borrow().contains_key(&id_str) {
                    id_str.push('\'');
                }
                self.variables_ids.borrow_mut().insert(id_str.clone(), id);
                id_str
            })
            .clone()
    }
}
