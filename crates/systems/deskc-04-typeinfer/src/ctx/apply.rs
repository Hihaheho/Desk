use hir::{expr::Expr, meta::WithMeta};

use crate::{
    ctx::Ctx,
    ctx::Log,
    error::{ExprTypeError, TypeError},
    substitute::substitute,
    to_expr_type_error,
    ty::Type,
};

impl Ctx {
    pub fn apply(&self, ty: &Type, expr: &WithMeta<Expr>) -> Result<(Ctx, Type), ExprTypeError> {
        let ret = match ty {
            Type::Label { label: _, item } => self.apply(item, expr)?,
            Type::Brand { brand: _, item } => self.apply(item, expr)?,
            Type::Function { parameter, body } => {
                let delta = self.check(expr, &*parameter)?.recover_effects();
                // if a type of expr is synthed, output can be substituded with the type.
                delta
                    .synth(expr)
                    .ok()
                    .map(|with| with.recover_effects())
                    .and_then(|(ctx, ty)| {
                        ctx.subtype(&ty, parameter).ok().map(|ctx| {
                            let ty = ctx.substitute_from_ctx(body);
                            (ctx, ty)
                        })
                    })
                    .unwrap_or((delta, *body.clone()))
            }
            Type::Existential(id) => {
                let a1 = self.fresh_existential();
                let a2 = self.fresh_existential();
                self.add(Log::Existential(a2))
                    .add(Log::Existential(a1))
                    .add(Log::TypedVariable(
                        *id,
                        Type::Function {
                            parameter: Box::new(Type::Existential(a1)),
                            body: Box::new(Type::Existential(a2)),
                        },
                    ))
                    .check(expr, &Type::Existential(a1))?
                    .recover_effects()
                    .with_type(Type::Existential(a2))
            }
            Type::ForAll { variable, body } => {
                let a = self.fresh_existential();
                self.add(Log::Existential(a))
                    .apply(&substitute(&*body, variable, &Type::Existential(a)), expr)?
            }
            _ => {
                return Err(to_expr_type_error(
                    expr,
                    TypeError::NotApplicable {
                        ty: ty.clone(),
                        expr: Box::new(expr.value.clone()),
                    },
                ))
            }
        };
        Ok(ret)
    }
}
