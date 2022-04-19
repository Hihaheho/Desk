use hir::{
    expr::{Expr, Literal},
    meta::WithMeta,
};

use crate::{
    error::ExprTypeError, to_expr_type_error, ty::Type, with_effects::WithEffects, ctx::Ctx, ctx::Log,
};

impl Ctx {
    pub fn check(&self, expr: &WithMeta<Expr>, ty: &Type) -> Result<WithEffects<Ctx>, ExprTypeError> {
        let scope = self.begin_scope();
        let mut effects = Vec::new();
        let ctx = match (&expr.value, ty) {
            (Expr::Literal(Literal::Int(_)), Type::Number) => self.clone(),
            (Expr::Literal(Literal::Float(_)), Type::Number) => self.clone(),
            (Expr::Literal(Literal::Rational(_, _)), Type::Number) => self.clone(),
            (Expr::Literal(Literal::String(_)), Type::String) => self.clone(),
            (
                Expr::Function {
                    parameter: _,
                    body: _,
                },
                Type::Function {
                    parameter: _ty_parameter,
                    body: _ty_body,
                },
            ) => {
                todo!()
            }
            (_, Type::ForAll { variable, body }) => {
                let WithEffects(ctx, effs) = self
                    .add(Log::Variable(*variable))
                    .check(expr, &*body)?
                    .recover_effects()
                    .truncate_from(&Log::Variable(*variable));
                effects.extend(effs);
                ctx
            }
            (_, ty) => {
                let (ctx, synthed) = self.synth(expr)?.recover_effects();
                ctx.subtype(
                    &ctx.substitute_from_ctx(&synthed),
                    &ctx.substitute_from_ctx(ty),
                )
                .map_err(|error| to_expr_type_error(expr, error))?
            }
        };
        let effects = ctx.end_scope(scope);
        let ty = ctx.substitute_from_ctx(ty);
        ctx.store_type_and_effects(expr, ty, effects.clone());
        Ok(WithEffects(ctx, effects))
    }
}
