use errors::typeinfer::{ExprTypeError, TypeError};
use hir::{
    expr::{Expr, Handler, Literal, MatchCase},
    meta::WithMeta,
};

use crate::{
    ctx::Ctx,
    ctx::Log,
    to_expr_type_error,
    internal_type::{effect_expr::EffectExpr, Effect, Type},
    utils::sum_all,
};

use super::{with_effects::WithEffects, with_type::WithType};

impl Ctx {
    pub fn synth(
        &self,
        expr: &WithMeta<Expr>,
    ) -> Result<WithEffects<WithType<Ctx>>, ExprTypeError> {
        let scope = self.begin_scope();
        let WithType(ctx, ty) = match &expr.value {
            Expr::Literal(Literal::Integer(_)) => self.clone().with_type(Type::Integer),
            Expr::Literal(Literal::Rational(_, _)) => self.clone().with_type(Type::Rational),
            Expr::Literal(Literal::Real(_)) => self.clone().with_type(Type::Real),
            Expr::Literal(Literal::String(_)) => self.clone().with_type(Type::String),
            Expr::Literal(Literal::Hole) => todo!(),
            Expr::Do { stmt, expr } => {
                let WithType(ctx, _) = self.synth(stmt)?.recover_effects();
                ctx.synth(expr)?.recover_effects()
            }
            Expr::Let {
                definition,
                expression,
            } => {
                let WithType(ctx, def_ty) = self.synth(definition)?.recover_effects();
                let ctx = ctx.make_polymorphic(definition, def_ty);
                let WithType(ctx, ty) = ctx.synth(expression)?.recover_effects();
                ctx.with_type(ty)
            }
            Expr::Perform { input, output } => {
                let WithType(ctx, ty) = self.synth(input)?.recover_effects();
                let output = ctx.save_from_hir_type(output);
                ctx.add(Log::Effect(EffectExpr::Effects(vec![Effect {
                    input: ty,
                    output: output.clone(),
                }])))
                .with_type(output)
            }
            Expr::Continue { input, output } => {
                let WithType(ctx, input_ty) = self.synth(input)?.recover_effects();
                let output = ctx.save_from_hir_type(output);
                let ctx = ctx
                    .subtype(
                        ctx.continue_output
                            .borrow()
                            .last()
                            .ok_or(TypeError::ContinueOutOfHandle)
                            .map_err(|error| to_expr_type_error(expr, error))?,
                        &output,
                    )
                    .map_err(|error| to_expr_type_error(expr, error))?;
                let continue_input = ctx.continue_input.borrow();
                ctx.subtype(
                    &input_ty,
                    continue_input
                        .last()
                        .ok_or(TypeError::ContinueOutOfHandle)
                        .map_err(|error| to_expr_type_error(expr, error))?,
                )
                .map_err(|error| to_expr_type_error(expr, error))?
                .add(Log::Effect(EffectExpr::Effects(vec![Effect {
                    input: input_ty,
                    output: output.clone(),
                }])))
                .with_type(output)
            }
            Expr::Handle { expr, handlers } => {
                // synth expr
                let WithEffects(WithType(mut ctx, expr_ty), mut expr_effects) = self.synth(expr)?;

                // push continue output type.
                ctx.continue_output.borrow_mut().push(expr_ty.clone());

                let mut handler_types = Vec::with_capacity(handlers.len());
                let mut handled_effects = Vec::with_capacity(handlers.len());
                let mut handler_effects = Vec::with_capacity(handlers.len());
                // check handler
                handlers
                    .iter()
                    .map(
                        |Handler {
                             input,
                             output,
                             handler,
                         }| {
                            let output = self.save_from_hir_type(output);
                            // push handler input type
                            ctx.continue_input.borrow_mut().push(output.clone());

                            let WithEffects(WithType(next_ctx, handler_type), effects) =
                                ctx.synth(handler)?;
                            ctx = next_ctx;

                            // pop handler input type
                            ctx.continue_input.borrow_mut().pop();

                            // handled effect and continue effect
                            let handled_effect = Effect {
                                input: ctx.save_from_hir_type(input),
                                output,
                            };
                            let continue_effect = Effect {
                                input: handled_effect.output.clone(),
                                output: ctx.substitute_from_ctx(&expr_ty),
                            };
                            handler_types.push(handler_type);
                            handled_effects.push(handled_effect);
                            handler_effects.push(
                                // remove continue effect
                                EffectExpr::Sub {
                                    minuend: Box::new(effects),
                                    subtrahend: Box::new(EffectExpr::Effects(vec![
                                        continue_effect,
                                    ])),
                                },
                            );
                            Ok(())
                        },
                    )
                    .collect::<Result<Vec<_>, _>>()?;

                // pop continue output type.
                ctx.continue_output.borrow_mut().pop();

                // remove handled effects
                expr_effects = EffectExpr::Sub {
                    minuend: Box::new(expr_effects),
                    subtrahend: Box::new(EffectExpr::Effects(handled_effects)),
                };

                // add remain effects to ctx
                handler_effects.push(expr_effects);
                expr_effects = EffectExpr::Add(handler_effects);

                // construct handler type
                handler_types.push(ctx.substitute_from_ctx(&expr_ty));
                let types = handler_types
                    .into_iter()
                    .map(|ty| ctx.substitute_from_ctx(&ty))
                    .collect();

                ctx.add_effects(&expr_effects)
                    .with_type(sum_all(&ctx, types))
            }
            Expr::Apply {
                function,
                link_name: _,
                arguments,
            } => {
                if arguments.is_empty() {
                    // Reference
                    let fun = self.save_from_hir_type(function);
                    if let Type::Variable(id) = fun {
                        self.clone().with_type(
                            self.get_typed_var(&id)
                                .map_err(|error| to_expr_type_error(expr, error))?,
                        )
                    } else {
                        self.clone().with_type(fun)
                    }
                } else {
                    // Normal application
                    let fun = match self.save_from_hir_type(function) {
                        Type::Variable(var) => self
                            .get_typed_var(&var)
                            .map_err(|error| to_expr_type_error(expr, error))?,
                        ty => ty,
                    };
                    let WithType(ctx, ty) = arguments.iter().try_fold(
                        WithType(self.clone(), fun.clone()),
                        |WithType(ctx, fun), arg| ctx.apply(&fun, arg),
                    )?;

                    ctx.add_effects(&EffectExpr::Apply {
                        function: Box::new(fun),
                        arguments: arguments
                            .iter()
                            .map(|arg| self.get_type(&arg.meta.id))
                            .collect(),
                    })
                    .with_type(ty)
                }
            }
            Expr::Product(exprs) => {
                let mut ctx = self.clone();
                let mut types = Vec::with_capacity(exprs.len());
                for expr in exprs {
                    let WithType(delta, ty) = ctx.synth(expr)?.recover_effects();
                    ctx = delta;
                    types.push(ty);
                }
                ctx.with_type(Type::Product(types))
            }
            Expr::Typed { ty, item: expr } => {
                let ty = self.save_from_hir_type(ty);
                self.check(expr, &ty)?.recover_effects().with_type(ty)
            }
            Expr::Function { parameter, body } => {
                if let Type::Variable(id) = self.save_from_hir_type(parameter) {
                    let a = self.fresh_existential();
                    let b = self.fresh_existential();
                    let WithEffects(ctx, effects) = self
                        .add(Log::Existential(a))
                        .add(Log::Existential(b))
                        .add(Log::TypedVariable(id, Type::Existential(a)))
                        .check(body, &Type::Existential(b))?
                        .recover_effects()
                        .truncate_from(&Log::TypedVariable(id, Type::Existential(a)));
                    // Function captures effects in its body
                    ctx.with_type(Type::Function {
                        parameter: Box::new(Type::Existential(a)),
                        body: Box::new(self.with_effects(Type::Existential(b), effects)),
                    })
                } else {
                    let WithType(ctx, ty) = self.synth(body)?.recover_effects();
                    ctx.with_type(Type::Function {
                        parameter: Box::new(self.save_from_hir_type(parameter)),
                        body: Box::new(ty),
                    })
                }
            }
            Expr::Vector(values) => {
                let mut types = vec![];
                values
                    .iter()
                    .try_fold(self.clone(), |ctx, value| {
                        let WithType(ctx, ty) = ctx.synth(value)?.recover_effects();
                        types.push(ty);
                        Ok(ctx)
                    })?
                    .with_type(Type::Vector(Box::new(Type::Sum(types))))
            }
            Expr::Map(elems) => {
                let mut keys = vec![];
                let mut values = vec![];
                elems
                    .iter()
                    .try_fold(self.clone(), |ctx, elem| {
                        let WithType(ctx, key) = ctx.synth(&elem.key)?.recover_effects();
                        keys.push(key);
                        let WithType(ctx, value) = ctx.synth(&elem.value)?.recover_effects();
                        values.push(value);
                        Ok(ctx)
                    })?
                    .with_type(Type::Map {
                        key: Box::new(Type::Sum(keys)),
                        value: Box::new(Type::Sum(values)),
                    })
            }
            Expr::Match { of, cases } => {
                let (ty, out): (Vec<_>, Vec<_>) = cases
                    .iter()
                    .map(|MatchCase { ty, expr }| {
                        Ok((
                            self.save_from_hir_type(ty),
                            self.synth(expr)?.recover_effects().1,
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .unzip();
                let ty = sum_all(self, ty);
                let out = sum_all(self, out);
                self.check(of, &ty)?.recover_effects().with_type(out)
            }
            Expr::Label { label, item: body } => {
                let WithType(ctx, ty) = self.synth(body)?.recover_effects();
                ctx.with_type(Type::Label {
                    label: label.clone(),
                    item: Box::new(ty),
                })
            }
            Expr::Brand { brand, item: body } => {
                let WithType(ctx, ty) = self.synth(body)?.recover_effects();
                ctx.with_type(Type::Brand {
                    brand: brand.clone(),
                    item: Box::new(ty),
                })
            }
        };
        let effects = ctx.end_scope(scope);
        let ty = ctx.substitute_from_ctx(&ty);
        ctx.store_type_and_effects(expr.meta.id.clone(), ty.clone(), effects.clone());
        Ok(WithEffects(WithType(ctx, ty), effects))
    }
}
