use hir::meta::WithMeta;

use crate::{
    ctx::Ctx,
    ty::{effect_expr::EffectExpr, Effect, Type},
};

impl Ctx {
    pub(crate) fn gen_from_hir_type(&self, ty: &WithMeta<hir::ty::Type>) -> Type {
        use hir::ty::Type::*;
        match &ty.value {
            Number => Type::Number,
            String => Type::String,
            Trait(_types) => todo!(),
            Effectful { ty, effects } => self.with_effects(
                self.gen_from_hir_type(ty),
                self.gen_from_hir_effect_expr(effects),
            ),
            Infer => Type::Infer(ty.meta.id),
            This => todo!(),
            Product(types) => {
                Type::Product(types.iter().map(|t| self.gen_from_hir_type(t)).collect())
            }
            Sum(types) => Type::Sum(types.iter().map(|t| self.gen_from_hir_type(t)).collect()),
            Function { parameter, body } => Type::Function {
                parameter: Box::new(self.gen_from_hir_type(parameter)),
                body: Box::new(self.gen_from_hir_type(body)),
            },
            Array(ty) => Type::Array(Box::new(self.gen_from_hir_type(ty))),
            Set(ty) => Type::Set(Box::new(self.gen_from_hir_type(ty))),
            Let { variable, body } => Type::ForAll {
                variable: *variable,
                body: Box::new(self.gen_from_hir_type(body)),
            },
            Variable(id) => Type::Variable(*id),
            BoundedVariable {
                bound: _,
                identifier: _,
            } => todo!(),
            Brand { brand, item } => Type::Brand {
                brand: brand.clone(),
                item: Box::new(self.gen_from_hir_type(item)),
            },
            Label { label, item } => Type::Label {
                label: label.clone(),
                item: Box::new(self.gen_from_hir_type(item)),
            },
        }
    }

    pub(crate) fn gen_from_hir_effect_expr(
        &self,
        effects: &WithMeta<hir::ty::EffectExpr>,
    ) -> EffectExpr {
        match &effects.value {
            hir::ty::EffectExpr::Effects(effects) => EffectExpr::Effects(
                effects
                    .iter()
                    .map(|e| Effect {
                        input: self.gen_from_hir_type(&e.value.input),
                        output: self.gen_from_hir_type(&e.value.output),
                    })
                    .collect(),
            ),
            hir::ty::EffectExpr::Add(effects) => EffectExpr::Add(
                effects
                    .iter()
                    .map(|e| self.gen_from_hir_effect_expr(e))
                    .collect(),
            ),
            hir::ty::EffectExpr::Sub {
                minuend,
                subtrahend,
            } => EffectExpr::Sub {
                minuend: Box::new(self.gen_from_hir_effect_expr(minuend)),
                subtrahend: Box::new(self.gen_from_hir_effect_expr(subtrahend)),
            },
            hir::ty::EffectExpr::Apply {
                function,
                arguments,
            } => EffectExpr::Apply {
                function: Box::new(self.gen_from_hir_type(function)),
                arguments: arguments
                    .iter()
                    .map(|a| self.gen_from_hir_type(a))
                    .collect(),
            },
        }
    }
}
