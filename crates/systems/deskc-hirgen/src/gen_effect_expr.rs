use ast::meta::WithMeta as AstWithMeta;
use hir::{
    meta::WithMeta,
    ty::{Effect, EffectExpr},
};

use crate::{error::HirGenError, HirGen};

impl HirGen {
    pub fn gen_effect_expr(
        &self,
        expr: &AstWithMeta<ast::ty::EffectExpr>,
    ) -> Result<WithMeta<EffectExpr>, HirGenError> {
        let AstWithMeta { value: expr, meta } = expr;
        self.push_meta(meta);
        let expr = match expr {
            ast::ty::EffectExpr::Effects(effects) => self.with_meta(EffectExpr::Effects(
                effects
                    .iter()
                    .map(
                        |AstWithMeta {
                             meta,
                             value: effect,
                         }| {
                            self.push_meta(meta);
                            Ok(self.with_meta(Effect {
                                input: self.gen_type(&effect.input)?,
                                output: self.gen_type(&effect.output)?,
                            }))
                        },
                    )
                    .collect::<Result<_, _>>()?,
            )),
            ast::ty::EffectExpr::Add(exprs) => self.with_meta(EffectExpr::Add(
                exprs
                    .iter()
                    .map(|e| self.gen_effect_expr(e))
                    .collect::<Result<_, _>>()?,
            )),
            ast::ty::EffectExpr::Sub {
                minuend,
                subtrahend,
            } => self.with_meta(EffectExpr::Sub {
                minuend: Box::new(self.gen_effect_expr(minuend)?),
                subtrahend: Box::new(self.gen_effect_expr(subtrahend)?),
            }),
            ast::ty::EffectExpr::Apply {
                function,
                arguments,
            } => self.with_meta(EffectExpr::Apply {
                function: Box::new(self.gen_type(function)?),
                arguments: arguments
                    .iter()
                    .map(|a| self.gen_type(a))
                    .collect::<Result<_, _>>()?,
            }),
        };
        Ok(expr)
    }
}
