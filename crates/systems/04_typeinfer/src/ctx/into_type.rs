use crate::{
    ty::{effect_expr::EffectExpr, Type},
    Ctx,
};

impl Ctx {
    pub(crate) fn into_type(&self, ty: &Type) -> types::Type {
        match ty {
            Type::Number => types::Type::Number,
            Type::String => types::Type::String,
            Type::Product(types) => {
                types::Type::product(types.into_iter().map(|t| self.into_type(&t)).collect())
            }
            Type::Sum(types) => {
                types::Type::sum(types.into_iter().map(|t| self.into_type(&t)).collect())
            }
            Type::Function { parameter, body } => {
                if let Type::Function { .. } = **body {
                    match self.into_type(body) {
                        types::Type::Function {
                            mut parameters,
                            body,
                        } => {
                            parameters.insert(0, self.into_type(parameter));
                            types::Type::function(parameters, *body)
                        }
                        _ => panic!(),
                    }
                } else {
                    types::Type::function(vec![self.into_type(parameter)], self.into_type(body))
                }
            }
            Type::Array(ty) => types::Type::Array(Box::new(self.into_type(ty))),
            Type::Set(ty) => types::Type::Set(Box::new(self.into_type(ty))),
            Type::Variable(id) => types::Type::Variable(*id),
            Type::ForAll { variable, body } => types::Type::ForAll {
                variable: *variable,
                body: Box::new(self.into_type(body)),
            },
            Type::Effectful { ty, effects } => types::Type::Effectful {
                ty: Box::new(self.into_type(&**ty)),
                effects: self.into_effect_expr(effects),
            },
            Type::Brand { brand, item } => types::Type::Brand {
                brand: brand.clone(),
                item: Box::new(self.into_type(item)),
            },
            Type::Label { label, item } => types::Type::Label {
                label: label.clone(),
                item: Box::new(self.into_type(item)),
            },
            Type::Existential(id) => self.into_type(
                self.types
                    .borrow()
                    .get(id)
                    .expect(&format!("should be instansiated: {}", id)),
            ),
            Type::Infer(id) => {
                self.into_type(self.types.borrow().get(id).expect("should be inferred"))
            }
        }
    }

    pub(crate) fn into_effect_expr(&self, expr: &EffectExpr) -> types::EffectExpr {
        match expr {
            EffectExpr::Effects(effects) => types::EffectExpr::Effects(
                effects
                    .iter()
                    .map(|e| types::Effect {
                        input: self.into_type(&e.input),
                        output: self.into_type(&e.output),
                    })
                    .collect(),
            ),
            EffectExpr::Add(effects) => {
                types::EffectExpr::Add(effects.iter().map(|e| self.into_effect_expr(e)).collect())
            }
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => types::EffectExpr::Sub {
                minuend: Box::new(self.into_effect_expr(minuend)),
                subtrahend: Box::new(self.into_effect_expr(subtrahend)),
            },
            EffectExpr::Apply {
                function,
                arguments,
            } => types::EffectExpr::Apply {
                function: Box::new(self.into_type(function)),
                arguments: arguments.iter().map(|a| self.into_type(a)).collect(),
            },
        }
    }
}
