use errors::typeinfer::{TypeError, TypeOrString};

use crate::{
    internal_type::{effect_expr::EffectExpr, Type},
    Ctx,
};

use super::Id;

impl Ctx {
    pub fn gen_type(&self, ty: &Type) -> Result<ty::Type, TypeError> {
        let ty = match ty {
            Type::Real => ty::Type::Real,
            Type::Rational => ty::Type::Rational,
            Type::Integer => ty::Type::Integer,
            Type::String => ty::Type::String,
            Type::Product(types) => ty::Type::Product(
                types
                    .iter()
                    .map(|t| self.gen_type(t))
                    .collect::<Result<_, _>>()?,
            ),
            Type::Sum(types) => ty::Type::Sum(
                types
                    .iter()
                    .map(|t| self.gen_type(t))
                    .collect::<Result<_, _>>()?,
            ),
            Type::Function { parameter, body } => ty::Type::Function(Box::new(ty::Function {
                parameter: self.gen_type(parameter)?,
                body: self.gen_type(body)?,
            })),
            Type::Vector(ty) => ty::Type::Vector(Box::new(self.gen_type(ty)?)),
            Type::Map { key, value } => ty::Type::Map {
                key: Box::new(self.gen_type(key)?),
                value: Box::new(self.gen_type(value)?),
            },
            Type::Variable(id) => ty::Type::Variable(self.get_ident_of(*id)),
            Type::ForAll {
                variable,
                bound,
                body,
            } => ty::Type::ForAll {
                variable: self.get_ident_of(*variable),
                bound: bound
                    .as_ref()
                    .map(|bound| Ok(Box::new(self.gen_type(bound)?)))
                    .transpose()?,
                body: Box::new(self.gen_type(body)?),
            },
            Type::Effectful { ty, effects } => ty::Type::Effectful {
                ty: Box::new(self.gen_type(ty)?),
                effects: self.gen_effect_expr(effects)?,
            },
            Type::Brand { brand, item } => ty::Type::Brand {
                brand: brand.clone(),
                item: Box::new(self.gen_type(item)?),
            },
            Type::Label { label, item } => ty::Type::Label {
                label: label.clone(),
                item: Box::new(self.gen_type(item)?),
            },
            Type::Existential(id) => self
                .types
                .borrow()
                .get(id)
                .map(|ty| self.gen_type(ty))
                .transpose()?
                .unwrap_or_else(|| ty::Type::Variable(self.get_ident_of(*id))),
            Type::Infer(id) => self.gen_type(
                self.ir_types
                    .borrow()
                    .get(id)
                    .ok_or_else(|| TypeError::NotInferred { id: id.clone() })?,
            )?,
        };
        Ok(ty)
    }

    pub fn gen_type_or_string(&self, ty: &Type) -> TypeOrString {
        if let Ok(ty) = self.gen_type(ty) {
            TypeOrString::Type(ty)
        } else {
            TypeOrString::String(format!("{ty:?}"))
        }
    }

    pub(crate) fn gen_effect_expr(&self, expr: &EffectExpr) -> Result<ty::EffectExpr, TypeError> {
        let expr = match expr {
            EffectExpr::Effects(effects) => ty::EffectExpr::Effects(
                effects
                    .iter()
                    .map(|e| {
                        Ok(ty::Effect {
                            input: self.gen_type(&e.input)?,
                            output: self.gen_type(&e.output)?,
                        })
                    })
                    .collect::<Result<_, _>>()?,
            ),
            EffectExpr::Add(effects) => ty::EffectExpr::Add(
                effects
                    .iter()
                    .map(|e| self.gen_effect_expr(e))
                    .collect::<Result<_, _>>()?,
            ),
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => ty::EffectExpr::Sub {
                minuend: Box::new(self.gen_effect_expr(minuend)?),
                subtrahend: Box::new(self.gen_effect_expr(subtrahend)?),
            },
            EffectExpr::Apply {
                function,
                arguments,
            } => ty::EffectExpr::Apply {
                function: Box::new(self.gen_type(function)?),
                arguments: arguments
                    .iter()
                    .map(|a| self.gen_type(a))
                    .collect::<Result<_, _>>()?,
            },
        };
        Ok(expr)
    }

    fn get_ident_of(&self, id: Id) -> String {
        self.variables_idents
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| {
                let ident = self.ident_gen.borrow_mut().next_ident();
                self.variables_ids.borrow_mut().insert(ident.clone(), id);
                ident
            })
            .clone()
    }
}
