use errors::typeinfer::ExprTypeError;
use hir::{
    expr::{Expr, Literal},
    meta::WithMeta,
};

use crate::{ctx::Ctx, ctx::Log, to_expr_type_error, ty::Type};

use super::{with_effects::WithEffects, with_type::WithType};

impl Ctx {
    pub fn check(
        &self,
        expr: &WithMeta<Expr>,
        ty: &Type,
    ) -> Result<WithEffects<Ctx>, ExprTypeError> {
        let scope = self.begin_scope();
        let mut synthed_ty = None;
        let ctx = match (&expr.value, ty) {
            (Expr::Literal(Literal::Integer(_)), Type::Real) => self.clone(),
            (Expr::Literal(Literal::Real(_)), Type::Real) => self.clone(),
            (Expr::Literal(Literal::Rational(_, _)), Type::Real) => self.clone(),
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
            (
                _,
                Type::ForAll {
                    variable,
                    bound,
                    body,
                },
            ) => self
                .add(Log::Variable(*variable))
                .check(expr, body)?
                .recover_effects()
                .bound_check(&Type::Variable(*variable), bound)
                .map_err(|error| to_expr_type_error(expr, error))?
                .truncate_from(&Log::Variable(*variable))
                .recover_effects(),
            (_, _) => {
                let WithType(ctx, synthed) = self.synth(expr)?.recover_effects();
                synthed_ty = Some(synthed.clone());
                ctx.subtype(
                    &ctx.substitute_from_ctx(&synthed),
                    &ctx.substitute_from_ctx(ty),
                )
                .map_err(|error| to_expr_type_error(expr, error))?
            }
        };
        let effects = ctx.end_scope(scope);
        let ty = synthed_ty.as_ref().unwrap_or(ty);
        ctx.store_type_and_effects(
            expr.meta.id.clone(),
            ctx.substitute_from_ctx(ty),
            effects.clone(),
        );
        Ok(WithEffects(ctx, effects))
    }
}
