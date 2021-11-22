mod error;
mod from_hir_type;
mod into_type;
mod mono_type;
mod occurs_in;
mod substitute;
mod substitute_from_ctx;
mod ty;
mod utils;
mod well_formed;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use error::TypeError;
use hir::{
    expr::{Expr, Literal, MatchCase},
    meta::WithMeta,
};
use mono_type::MonoType;
use occurs_in::OccursIn;
use substitute::Substitute;
use substitute_from_ctx::SubstituteFromCtx;
use ty::{Effect, Type, TypeVisitor, TypeVisitorMut};
use types::{IdGen, Types};
use well_formed::WellFormed;

use crate::utils::{sum_all, with_effects};

pub type Id = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Log {
    Variable(Id),
    TypedVariable(Id, Type),
    Existential(Id),
    Solved(Id, Type),
    Marker(Id),
    Effect(Effect),
}

#[derive(Default, Debug, Clone)]
pub struct Ctx {
    id_gen: Rc<RefCell<IdGen>>,
    logs: RefCell<Vec<Log>>,
    // Result of type inference
    types: Rc<RefCell<HashMap<Id, Type>>>,
}

pub struct WithEffects<T>(T, Vec<Effect>);

type CtxWithEffects = WithEffects<Ctx>;

impl CtxWithEffects {
    fn recover_effects(self) -> Ctx {
        let effects = self.1.into_iter().map(|effect| Log::Effect(effect));
        self.0.logs.borrow_mut().extend(effects);
        self.0
    }
}

impl Ctx {
    fn empty(&self) -> Self {
        Self {
            id_gen: self.id_gen.clone(),
            logs: Default::default(),
            types: self.types.clone(),
        }
    }

    fn begin_scope(&self) -> Id {
        let id = self.fresh_existential();
        self.logs.borrow_mut().push(Log::Marker(id));
        id
    }

    fn store_type_and_effects<T>(&self, expr: &WithMeta<T>, ty: &Type, effects: Vec<Effect>) {
        if let Some(meta) = &expr.meta {
            self.store_type(meta.id, with_effects(self.substitute_from_ctx(ty), effects));
        }
    }

    fn store_type(&self, id: Id, ty: Type) {
        let mut types = self.types.borrow_mut();
        match types.get(&id) {
            None | Some(&Type::Existential(_)) => {
                types.insert(id, ty);
            }
            // Keeps the generic one
            _ => {}
        }
    }

    fn from_hir_type(&self, hir_ty: &WithMeta<hir::ty::Type>) -> Type {
        let ty = from_hir_type::from_hir_type(self, hir_ty);
        let ty = self.substitute_from_ctx(&ty);
        self.store_type_and_effects(&hir_ty, &ty, vec![]);
        ty
    }

    pub fn get_id_gen(&self) -> IdGen {
        self.id_gen.borrow().clone()
    }

    pub fn get_types(&self) -> Types {
        Types {
            types: self
                .types
                .borrow()
                .iter()
                .map(|(id, ty)| (id.clone(), self.into_type(ty)))
                .collect(),
        }
    }

    pub fn into_type(&self, ty: &Type) -> types::Type {
        into_type::into_type(self, ty)
    }

    fn end_scope(&self, scope: Id) -> Vec<Effect> {
        let index = self.index(&Log::Marker(scope)).expect("scope should exist");
        let mut effects = Vec::new();
        for log in &self.logs.borrow()[index..] {
            match log {
                Log::Effect(effect) => effects.push(self.substitute_from_context_effect(effect)),
                _ => (),
            }
        }
        // Delete scope
        self.logs.borrow_mut().remove(index);
        effects
    }

    fn index(&self, log: &Log) -> Option<usize> {
        self.logs.borrow().iter().position(|x| x == log)
    }

    fn fresh_existential(&self) -> Id {
        self.id_gen.borrow_mut().next_id()
    }

    fn with_type(self, ty: Type) -> (Self, Type) {
        (self, ty)
    }

    fn add(&self, log: Log) -> Ctx {
        let cloned = self.clone();
        cloned.logs.borrow_mut().push(log);
        cloned
    }

    fn insert_in_place(&self, log: &Log, logs: Vec<Log>) -> Ctx {
        let cloned = self.clone();
        let index = cloned.index(log).expect(&format!(
            "{:?}: log not found: {:?} to be replaced {:?}",
            self.logs, log, logs
        ));
        cloned.logs.borrow_mut().splice(index..=index, logs);
        cloned
    }

    fn truncate_from(&self, log: &Log) -> CtxWithEffects {
        let cloned = self.clone();
        let index = self.index(log).expect(&format!(
            "{:?}: log not found: {:?} to be truncated",
            self.logs.borrow(),
            log
        ));

        let tail_ctx = self.empty();
        let mut effects = Vec::new();
        cloned
            .logs
            .borrow_mut()
            .splice(index.., vec![])
            .for_each(|tail| match tail {
                Log::Effect(effect) => {
                    effects.push(tail_ctx.substitute_from_context_effect(&effect))
                }
                log => tail_ctx.logs.borrow_mut().push(log.clone()),
            });

        WithEffects(cloned, effects)
    }

    fn has_variable(&self, id: &Id) -> bool {
        self.logs
            .borrow()
            .iter()
            .any(|log| log == &Log::Variable(*id))
    }

    fn has_existential(&self, id: &Id) -> bool {
        self.logs
            .borrow()
            .iter()
            .any(|log| log == &Log::Existential(*id))
    }

    fn get_solved(&self, id: &Id) -> Option<Type> {
        self.logs.borrow().iter().find_map(|log| match log {
            Log::Solved(var, ty) if var == id => Some(ty.clone()),
            _ => None,
        })
    }

    fn get_typed_var(&self, id: &Id) -> Result<Type, TypeError> {
        self.logs
            .borrow()
            .iter()
            .find_map(|log| {
                if let Log::TypedVariable(typed_id, ty) = log {
                    if *typed_id == *id {
                        Some(ty)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .cloned()
            .ok_or(TypeError::VariableNotTyped { id: *id })
    }

    fn check_and_effects(
        &self,
        expr: &WithMeta<Expr>,
        ty: &Type,
    ) -> Result<WithEffects<Ctx>, TypeError> {
        let scope = self.begin_scope();
        let ctx = match (&expr.value, ty) {
            (Expr::Literal(Literal::Int(_)), Type::Number) => self.clone(),
            (Expr::Literal(Literal::Float(_)), Type::Number) => self.clone(),
            (Expr::Literal(Literal::Rational(_, _)), Type::Number) => self.clone(),
            (Expr::Literal(Literal::String(_)), Type::String) => self.clone(),
            (
                Expr::Function { parameter, body },
                Type::Function {
                    parameter: ty_parameter,
                    body: ty_body,
                },
            ) => {
                todo!()
            }
            (_, Type::ForAll { variable, body }) => {
                self.add(Log::Variable(*variable)).check(expr, &*body)?
            }
            (_, ty) => {
                let (ctx, synthed) = self.synth(expr)?;
                ctx.subtype(
                    &ctx.substitute_from_ctx(&synthed),
                    &ctx.substitute_from_ctx(ty),
                )?
            }
        };
        let effects = ctx.end_scope(scope);
        ctx.store_type_and_effects(expr, ty, effects.clone());
        Ok(WithEffects(ctx, effects))
    }

    fn check(&self, expr: &WithMeta<Expr>, ty: &Type) -> Result<Ctx, TypeError> {
        self.check_and_effects(expr, ty)
            .map(|with_effects| with_effects.0)
    }

    pub fn synth_and_effects(
        &self,
        expr: &WithMeta<Expr>,
    ) -> Result<WithEffects<(Ctx, Type)>, TypeError> {
        let scope = self.begin_scope();
        let (ctx, ty) = match &expr.value {
            Expr::Literal(Literal::Int(_)) => (self.clone(), Type::Number),
            Expr::Literal(Literal::Float(_)) => (self.clone(), Type::Number),
            Expr::Literal(Literal::Rational(_, _)) => (self.clone(), Type::Number),
            Expr::Literal(Literal::String(_)) => (self.clone(), Type::String),
            Expr::Literal(Literal::Hole) => todo!(),
            Expr::Let {
                ty,
                definition,
                expression,
            } => {
                if let WithMeta {
                    value: hir::ty::Type::Variable(var),
                    meta: _,
                } = &ty
                {
                    // TODO: support let rec
                    let (ctx, def_ty) = self.synth(&definition)?;
                    let (ctx, ty) = ctx
                        .add(Log::TypedVariable(*var, def_ty.clone()))
                        .synth(&expression)?;
                    ctx.insert_in_place(&Log::TypedVariable(*var, def_ty), vec![])
                        .with_type(ty)
                } else {
                    let WithEffects((_ctx, _ty), effects) = self.synth_and_effects(&definition)?;
                    let (ctx, ty) = self.synth(&expression)?;
                    ctx.with_effects(&effects).with_type(ty)
                }
            }
            Expr::Perform { input, output } => {
                let (ctx, ty) = self.synth(&input)?;
                let output = ctx.from_hir_type(output);
                ctx.add(Log::Effect(Effect {
                    input: ty,
                    output: output.clone(),
                }))
                .with_type(output)
            }
            Expr::Handle {
                input,
                output,
                handler,
                expr,
            } => {
                // synth expr
                let WithEffects((ctx, expr_ty), mut expr_effects) = self.synth_and_effects(expr)?;
                expr_effects
                    .iter_mut()
                    .for_each(|effect| *effect = ctx.substitute_from_context_effect(&effect));
                // check handler
                let WithEffects(ctx, mut handler_effects) =
                    self.check_and_effects(handler, &expr_ty)?;
                handler_effects
                    .iter_mut()
                    .for_each(|effect| *effect = ctx.substitute_from_context_effect(&effect));
                // handled effect and continue effect
                let handled_effect = self.substitute_from_context_effect(&Effect {
                    input: self.from_hir_type(input),
                    output: self.from_hir_type(output),
                });
                let continue_effect = self.substitute_from_context_effect(&Effect {
                    input: handled_effect.output.clone(),
                    output: expr_ty.clone(),
                });
                // remove handled effect
                let position = (&expr_effects)
                    .iter()
                    .position(|effect| effect == &handled_effect)
                    .ok_or(TypeError::UnknownEffectHandled {
                        effect: handled_effect,
                    })?;
                expr_effects.remove(position);
                // remove continue effect
                if let Some(position) = dbg!(&handler_effects)
                    .iter()
                    .position(|effect| effect == &continue_effect)
                {
                    handler_effects.remove(position);
                }
                // add remain effects to ctx
                expr_effects.extend(handler_effects);
                self.with_effects(&expr_effects).with_type(expr_ty)
            }
            Expr::Apply {
                function,
                arguments,
            } => {
                if arguments.is_empty() {
                    // Reference
                    let fun = self.from_hir_type(function);
                    if let Type::Variable(id) = fun {
                        self.clone().with_type(self.get_typed_var(&id)?)
                    } else {
                        self.clone().with_type(fun)
                    }
                } else {
                    // Normal application
                    let fun = match self.from_hir_type(function) {
                        Type::Variable(var) => self.get_typed_var(&var)?,
                        ty => ty,
                    };
                    arguments
                        .iter()
                        .try_fold((self.clone(), fun), |(ctx, fun), arg| ctx.apply(&fun, &arg))?
                }
            }
            Expr::Product(exprs) => {
                let mut ctx = self.clone();
                let mut types = Vec::with_capacity(exprs.len());
                for expr in exprs {
                    let (delta, ty) = ctx.synth(expr)?;
                    ctx = delta;
                    types.push(ty);
                }
                ctx.with_type(Type::Product(types))
            }
            Expr::Typed { ty, expr } => {
                let ty = self.from_hir_type(ty);
                self.check(&expr, &ty)?.with_type(ty)
            }
            Expr::Function { parameter, body } => {
                if let Type::Variable(id) = self.from_hir_type(parameter) {
                    let a = self.fresh_existential();
                    let b = self.fresh_existential();
                    let WithEffects(ctx, effects) = self
                        .add(Log::Existential(a))
                        .add(Log::Existential(b))
                        .add(Log::TypedVariable(id, Type::Existential(a)))
                        .check(&body, &Type::Existential(b))?
                        .truncate_from(&Log::TypedVariable(id, Type::Existential(a)));
                    // Function captures effects in its body
                    ctx.with_type(Type::Function {
                        parameter: Box::new(Type::Existential(a)),
                        body: Box::new(with_effects(Type::Existential(b), effects)),
                    })
                } else {
                    let a = self.fresh_existential();
                    self.add(Log::Existential(a))
                        .check(&body, &Type::Existential(a))?
                        .with_type(Type::Function {
                            parameter: Box::new(self.from_hir_type(parameter)),
                            body: Box::new(Type::Existential(a)),
                        })
                }
            }
            Expr::Array(_) => todo!(),
            Expr::Set(_) => todo!(),
            Expr::Match { of, cases } => {
                let (ty, out): (Vec<_>, Vec<_>) = cases
                    .iter()
                    .map(|MatchCase { ty, expr }| Ok((self.from_hir_type(ty), self.synth(expr)?.1)))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .unzip();
                let ty = dbg!(sum_all(self, ty));
                let out = dbg!(sum_all(self, out));
                self.check(&*of, &ty)?.with_type(out)
            }
        };
        let effects = ctx.end_scope(scope);
        ctx.store_type_and_effects(expr, &ty, effects.clone());
        Ok(WithEffects((ctx, ty), effects))
    }

    pub fn synth(&self, expr: &WithMeta<Expr>) -> Result<(Ctx, Type), TypeError> {
        self.synth_and_effects(expr)
            .map(|with_effects| with_effects.0)
    }

    fn apply(&self, ty: &Type, expr: &WithMeta<Expr>) -> Result<(Ctx, Type), TypeError> {
        let ret = match ty {
            Type::Label { label: _, item } => self.apply(item, expr)?,
            Type::Brand { brand: _, item } => self.apply(item, expr)?,
            Type::Function { parameter, body } => {
                let delta = self.check(expr, &*parameter)?;
                // if a type of expr is synthed, output can be substituded with the type.
                let ty = self
                    .synth(expr)
                    .ok()
                    .and_then(|(ctx, ty)| {
                        ctx.subtype(&ty, parameter)
                            .ok()
                            .map(|ctx| ctx.substitute_from_ctx(body))
                    })
                    .unwrap_or(*body.clone());
                // If output is effectful, add them to the ctx.
                if let Type::Effectful { ty, effects } = &ty {
                    delta.with_effects(effects).with_type(*ty.clone())
                } else {
                    (delta, ty)
                }
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
                    .with_type(Type::Existential(a2))
            }
            Type::ForAll { variable, body } => self.add(Log::Existential(*variable)).apply(
                &substitute(&*body, variable, &Type::Existential(*variable)),
                expr,
            )?,
            _ => Err(TypeError::NotApplicable {
                ty: ty.clone(),
                expr: expr.value.clone(),
            })?,
        };
        Ok(ret)
    }

    fn is_well_formed(&self, ty: &Type) -> bool {
        let mut well_formed = WellFormed {
            ctx: self,
            well_formed: true,
        };
        well_formed.visit(ty);
        well_formed.well_formed
    }

    fn subtype(&self, sub: &Type, ty: &Type) -> Result<Ctx, TypeError> {
        let subtype_if = |pred: bool| {
            if pred {
                Ok(self.clone())
            } else {
                Err(TypeError::NotSubtype {
                    sub: sub.clone(),
                    ty: ty.clone(),
                })
            }
        };
        let ctx = match (sub, ty) {
            (Type::Variable(id), Type::Variable(id2)) if id == id2 => self.clone(),
            (Type::Existential(id), Type::Existential(id2)) if id == id2 => self.clone(),
            (Type::Number, Type::Number) => self.clone(),
            (Type::String, Type::String) => self.clone(),
            // TODO: return multi pass for error recovery?
            (Type::Product(sub_types), ty) => sub_types
                .iter()
                .find_map(|sub_ty| match self.subtype(sub_ty, ty) {
                    Ok(ctx) => Some(ctx),
                    Err(_) => None,
                })
                .ok_or(TypeError::NotSubtype {
                    sub: sub.clone(),
                    ty: ty.clone(),
                })?,
            // TODO: return multi pass for error recovery?
            (sub, Type::Sum(types)) => types
                .iter()
                .find_map(|ty| match self.subtype(sub, ty) {
                    Ok(ctx) => Some(ctx),
                    Err(_) => None,
                })
                .ok_or(TypeError::NotSubtype {
                    sub: sub.clone(),
                    ty: ty.clone(),
                })?,
            (
                Type::Function {
                    parameter: sub_parameter,
                    body: sub_body,
                },
                Type::Function { parameter, body },
            ) => {
                let theta = self.subtype(sub_parameter, parameter)?;
                theta.subtype(
                    &theta.substitute_from_ctx(body),
                    &theta.substitute_from_ctx(sub_body),
                )?
            }
            (Type::Array(sub), Type::Array(ty)) => self.subtype(sub, ty)?,
            (Type::Set(sub), Type::Set(ty)) => self.subtype(sub, ty)?,
            (Type::Variable(id), Type::Variable(id2)) => subtype_if(id == id2)?,
            (Type::ForAll { variable, body }, ty) => {
                let a = self.fresh_existential();
                self.add(Log::Marker(a))
                    .add(Log::Existential(a))
                    .subtype(&substitute(body, variable, &Type::Existential(a)), ty)?
                    .truncate_from(&Log::Marker(a))
                    .recover_effects()
            }
            (sub, Type::ForAll { variable, body }) => self
                .add(Log::Variable(*variable))
                .subtype(sub, body)?
                .truncate_from(&Log::Variable(*variable))
                .recover_effects(),
            (Type::Existential(id), ty) => {
                if occurs_in(id, ty) {
                    Err(TypeError::CircularExistential {
                        id: *id,
                        ty: ty.clone(),
                    })?
                } else {
                    self.instantiate_subtype(id, ty)?
                }
            }
            (sub, Type::Existential(id)) => {
                if occurs_in(id, sub) {
                    Err(TypeError::CircularExistential {
                        id: *id,
                        ty: ty.clone(),
                    })?
                } else {
                    self.instantiate_supertype(sub, id)?
                }
            }

            // handling labels must be under the instantiations
            (sub, Type::Label { item, label: _ }) => self.subtype(sub, item)?,
            (Type::Label { item, label: _ }, sup) => self.subtype(item, sup)?,
            (Type::Brand { item, brand: _ }, sup) => self.subtype(item, sup)?,

            (Type::Infer(id), sup) => {
                self.store_type(*id, sup.clone());
                self.clone()
            }
            (sub, Type::Infer(id)) => {
                self.store_type(*id, sub.clone());
                self.clone()
            }

            (
                Type::Effectful { ty, effects },
                Type::Effectful {
                    ty: ty2,
                    effects: super_effects,
                },
            ) => {
                let theta = self.subtype(ty, ty2)?;

                // get effects of super type
                let super_effects: Vec<_> = super_effects
                    .iter()
                    .map(|effect| theta.substitute_from_context_effect(effect))
                    .collect();
                // add effects to ctx that super type does not have
                let effects = effects
                    .into_iter()
                    .filter(|effect| !super_effects.contains(&effect));
                self.with_effects(effects)
            }
            (Type::Effectful { ty, effects }, ty2) => {
                let theta = self.subtype(ty, ty2)?;
                theta.with_effects(effects)
            }
            (sub, Type::Effectful { ty, effects: _ }) => self.subtype(sub, ty)?,
            (_, _) => Err(TypeError::NotSubtype {
                sub: sub.clone(),
                ty: ty.clone(),
            })?,
        };
        Ok(ctx)
    }

    fn instantiate_subtype(&self, id: &Id, sup: &Type) -> Result<Ctx, TypeError> {
        // In here, we can assume the context contains the existential type.
        let ctx = if is_monotype(sup)
            && self
                .truncate_from(&Log::Existential(*id))
                .recover_effects()
                .is_well_formed(sup)
        {
            self.insert_in_place(&Log::Existential(*id), vec![Log::Solved(*id, sup.clone())])
        } else {
            match sup {
                Type::Effectful { ty, effects: _ } => self.instantiate_subtype(id, ty)?,
                Type::Function { parameter, body } => {
                    let a1 = self.fresh_existential();
                    let a2 = self.fresh_existential();
                    let theta = self
                        .insert_in_place(
                            &Log::Existential(*id),
                            vec![
                                Log::Existential(a2),
                                Log::Existential(a1),
                                Log::Solved(
                                    *id,
                                    Type::Function {
                                        parameter: Box::new(Type::Existential(a1)),
                                        body: Box::new(Type::Existential(a2)),
                                    },
                                ),
                            ],
                        )
                        .instantiate_supertype(parameter, &a1)?;
                    theta.instantiate_subtype(&a2, &theta.substitute_from_ctx(&body))?
                }
                Type::ForAll { variable, body } => self
                    .add(Log::Variable(*variable))
                    .instantiate_subtype(id, body)?
                    .truncate_from(&Log::Variable(*variable))
                    .recover_effects(),
                Type::Existential(var) => self.insert_in_place(
                    &Log::Existential(*var),
                    vec![Log::Solved(*var, Type::Existential(*id))],
                ),
                Type::Product(types) => self.instantiate_composite_type_vec(
                    *id,
                    types,
                    Type::Product,
                    |ctx, id, sup| ctx.instantiate_subtype(id, sup),
                )?,
                Type::Sum(types) => {
                    self.instantiate_composite_type_vec(*id, types, Type::Sum, |ctx, id, sup| {
                        ctx.instantiate_subtype(id, sup)
                    })?
                }
                Type::Array(ty) => {
                    let a = self.fresh_existential();
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![
                            Log::Existential(a),
                            Log::Solved(*id, Type::Array(Box::new(Type::Existential(a)))),
                        ],
                    )
                    .instantiate_subtype(&a, ty)?
                }
                Type::Set(ty) => {
                    let a = self.fresh_existential();
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![
                            Log::Existential(a),
                            Log::Solved(*id, Type::Set(Box::new(Type::Existential(a)))),
                        ],
                    )
                    .instantiate_subtype(&a, ty)?
                }
                ty => Err(TypeError::NotInstantiable { ty: ty.clone() })?,
            }
        };
        self.store_type(*id, sup.clone());
        Ok(ctx)
    }

    fn instantiate_supertype(&self, sub: &Type, id: &Id) -> Result<Ctx, TypeError> {
        // In here, we can assume the context contains the existential type.
        let ctx = if is_monotype(sub)
            && self
                .truncate_from(&Log::Existential(*id))
                .recover_effects()
                .is_well_formed(sub)
        {
            self.insert_in_place(&Log::Existential(*id), vec![Log::Solved(*id, sub.clone())])
        } else {
            match sub {
                Type::Effectful { ty, effects } => effects
                    .iter()
                    .fold(self.instantiate_supertype(ty, id)?, |ctx, effect| {
                        ctx.add(Log::Effect(effect.clone()))
                    }),
                Type::Function { parameter, body } => {
                    let a1 = self.fresh_existential();
                    let a2 = self.fresh_existential();
                    let theta = self
                        .insert_in_place(
                            &Log::Existential(*id),
                            vec![
                                Log::Existential(a2),
                                Log::Existential(a1),
                                Log::Solved(
                                    *id,
                                    Type::Function {
                                        parameter: Box::new(Type::Existential(a1)),
                                        body: Box::new(Type::Existential(a2)),
                                    },
                                ),
                            ],
                        )
                        .instantiate_subtype(&a1, parameter)?;
                    theta.instantiate_supertype(&theta.substitute_from_ctx(&body), &a2)?
                }
                Type::ForAll { variable, body } => self
                    .add(Log::Marker(*variable))
                    .add(Log::Existential(*variable))
                    .instantiate_supertype(
                        &substitute(body, variable, &Type::Existential(*variable)),
                        id,
                    )?
                    .truncate_from(&Log::Marker(*variable))
                    .recover_effects(),
                Type::Existential(var) => self.insert_in_place(
                    &Log::Existential(*var),
                    vec![Log::Solved(*var, Type::Existential(*id))],
                ),
                Type::Product(types) => self.instantiate_composite_type_vec(
                    *id,
                    types,
                    Type::Product,
                    |ctx, id, sub| ctx.instantiate_supertype(sub, id),
                )?,
                Type::Sum(types) => {
                    self.instantiate_composite_type_vec(*id, types, Type::Sum, |ctx, id, sub| {
                        ctx.instantiate_supertype(sub, id)
                    })?
                }
                Type::Array(ty) => {
                    let a = self.fresh_existential();
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![
                            Log::Existential(a),
                            Log::Solved(*id, Type::Array(Box::new(Type::Existential(a)))),
                        ],
                    )
                    .instantiate_supertype(ty, &a)?
                }
                Type::Set(ty) => {
                    let a = self.fresh_existential();
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![
                            Log::Existential(a),
                            Log::Solved(*id, Type::Set(Box::new(Type::Existential(a)))),
                        ],
                    )
                    .instantiate_supertype(ty, &a)?
                }
                ty => Err(TypeError::NotInstantiable { ty: ty.clone() })?,
            }
        };
        self.store_type(*id, sub.clone());
        Ok(ctx)
    }

    fn instantiate_composite_type_vec(
        &self,
        id: Id,
        types: &Vec<Type>,
        f: fn(Vec<Type>) -> Type,
        instantiate: fn(&Ctx, &Id, &Type) -> Result<Ctx, TypeError>,
    ) -> Result<Ctx, TypeError> {
        let variables = types
            .iter()
            .map(|_| self.fresh_existential())
            .collect::<Vec<_>>();
        types.iter().zip(variables.iter()).try_fold(
            self.insert_in_place(
                &Log::Existential(id),
                variables
                    .iter()
                    .rev()
                    .map(|a| Log::Existential(*a))
                    .chain(vec![Log::Solved(
                        id,
                        f(variables
                            .iter()
                            .cloned()
                            .map(|a| Type::Existential(a))
                            .collect()),
                    )])
                    .collect(),
            ),
            |ctx, (ty, id)| instantiate(&ctx, id, ty),
        )
    }

    pub fn substitute_from_ctx(&self, a: &Type) -> Type {
        let mut substitute_from_ctx = SubstituteFromCtx { ctx: self };
        let mut a = a.clone();
        substitute_from_ctx.visit(&mut a);
        a
    }

    fn substitute_from_context_effect(&self, Effect { input, output }: &Effect) -> Effect {
        Effect {
            input: self.substitute_from_ctx(input),
            output: self.substitute_from_ctx(output),
        }
    }

    fn with_effects<'a>(&self, effects: impl IntoIterator<Item = &'a Effect>) -> Ctx {
        effects
            .into_iter()
            .map(|effect| self.substitute_from_context_effect(effect))
            .fold(self.clone(), |ctx, effect| ctx.add(Log::Effect(effect)))
    }
}

fn substitute(to: &Type, id: &Id, by: &Type) -> Type {
    let mut substitute = Substitute {
        id: *id,
        ty: by.clone(),
    };
    let mut to = to.clone();
    substitute.visit(&mut to);
    to
}

// existential type is occurs in the type
fn occurs_in(id: &Id, ty: &Type) -> bool {
    let mut occurs_in = OccursIn {
        id: *id,
        occurs_in: false,
    };
    occurs_in.visit(ty);
    occurs_in.occurs_in
}

fn is_monotype(ty: &Type) -> bool {
    let mut monotype = MonoType { is_monotype: true };
    monotype.visit(ty);
    monotype.is_monotype
}

#[cfg(test)]
mod tests {
    use file::FileId;
    use hir::meta::no_meta;
    use hirgen::HirGen;
    use pretty_assertions::assert_eq;

    use super::*;

    fn synth(expr: WithMeta<Expr>) -> Result<Type, TypeError> {
        let (ctx, ty) = Ctx {
            id_gen: Rc::new(RefCell::new(IdGen { next: 100 })),
            ..Default::default()
        }
        .synth(&expr)?;
        Ok(ctx.substitute_from_ctx(&ty))
    }

    fn parse(input: &str) -> WithMeta<Expr> {
        parse_inner(input).1
    }

    fn parse_inner(input: &str) -> (HirGen, WithMeta<Expr>) {
        use chumsky::prelude::end;
        use chumsky::{Parser, Stream};
        let expr = parser::expr::parser()
            .then_ignore(end())
            .parse(Stream::from_iter(
                input.len()..input.len() + 1,
                lexer::lexer()
                    .then_ignore(end())
                    .parse(input)
                    .unwrap()
                    .into_iter(),
            ))
            .unwrap();
        let gen = hirgen::HirGen::default();
        gen.push_file_id(FileId(0));
        let expr = gen.gen(&expr).unwrap();
        (gen, expr)
    }

    fn get_types(hirgen: &HirGen, ctx: &Ctx) -> Vec<(usize, Type)> {
        let attrs: HashMap<String, Id> = hirgen
            .attrs
            .borrow()
            .iter()
            .flat_map(|(id, attrs)| attrs.iter().map(|attr| (format!("{:?}", attr), id.clone())))
            .collect();
        (1usize..)
            .map_while(|i| {
                attrs
                    .get(&format!("{:?}", Expr::Literal(Literal::Int(i as i64))))
                    .and_then(|id| ctx.types.borrow().get(id).cloned())
                    .map(|ty| (i, ty))
            })
            .collect()
    }

    #[test]
    fn number() {
        assert_eq!(
            synth(no_meta(Expr::Literal(Literal::Int(1)))),
            Ok(Type::Number)
        );
    }

    #[test]
    fn function() {
        assert_eq!(
            synth(no_meta(Expr::Apply {
                function: no_meta(hir::ty::Type::Function {
                    parameter: Box::new(no_meta(hir::ty::Type::Number)),
                    body: Box::new(no_meta(hir::ty::Type::String)),
                }),
                arguments: vec![no_meta(Expr::Literal(Literal::Int(1))),]
            })),
            Ok(Type::String)
        );
    }

    #[test]
    fn let_() {
        assert_eq!(
            synth(parse(
                r#"
                    $ 1 ~ <'number>
            "#
            )),
            Ok(Type::Number)
        );
    }

    #[test]
    fn let_with_type() {
        assert_eq!(
            synth(parse(
                r#"
                    $ 1: <x> ~ <x>
            "#
            )),
            Ok(Type::Number)
        );
    }

    #[test]
    fn generic_function() {
        assert_eq!(
            synth(parse(
                r#"
                    \ <x> -> <x>
            "#
            )),
            Ok(Type::Function {
                parameter: Box::new(Type::Existential(101)),
                body: Box::new(Type::Existential(101)),
            })
        );
    }

    #[test]
    fn let_function() {
        assert_eq!(
            synth(parse(
                r#"
                    $ \ <x> -> <x>: <id> ~
                    <id> 1
            "#
            )),
            Ok(Type::Number)
        );
    }

    #[test]
    fn typing_expressions() {
        let (hirgen, expr) = parse_inner(
            r#"
            #1 $ #2 \ <x> -> #3 <x>: <id> ~
            $ #4 <id> #5 1 ~
            #6 <id> #7 "a"
        "#,
        );
        let ctx = Ctx::default();
        let (ctx, _ty) = ctx.synth(&expr).unwrap();

        assert_eq!(
            get_types(&hirgen, &ctx),
            vec![
                (1, Type::String),
                (
                    2,
                    Type::Function {
                        parameter: Box::new(Type::Existential(2)),
                        body: Box::new(Type::Existential(2)),
                    },
                ),
                (3, Type::Existential(2)),
                (4, Type::Number),
                (5, Type::Number),
                (6, Type::String),
                (7, Type::String),
            ],
        );
    }

    #[test]
    fn subtyping_sum_in_product() {
        let (hirgen, expr) = parse_inner(
            r#"
            $ #1 \ < + 'number, *. > -> 1: <fun> ~
            #3 <fun> #2 * 1, "a"
        "#,
        );
        let ctx = Ctx::default();
        let (ctx, _ty) = ctx.synth(&expr).unwrap();

        assert_eq!(
            get_types(&hirgen, &ctx),
            vec![
                (
                    1,
                    Type::Function {
                        parameter: Box::new(Type::Sum(vec![Type::Number, Type::Product(vec![])])),
                        body: Box::new(Type::Number),
                    },
                ),
                (2, Type::Product(vec![Type::Number, Type::String])),
                (3, Type::Number),
            ],
        );
    }

    #[test]
    fn perform() {
        let (hirgen, expr) = parse_inner(
            r#"
            $ #3 \ <x> -> #2 < \ 'number -> 'number > #1 ! <x> => <'number>: <fun> ~
            #4 <fun> "a"
        "#,
        );
        let ctx = Ctx::default();
        let (ctx, _ty) = ctx.synth(&expr).unwrap();

        dbg!(&ctx.logs);

        assert_eq!(
            get_types(&hirgen, &ctx),
            vec![
                (
                    1,
                    Type::Effectful {
                        ty: Box::new(Type::Number),
                        effects: vec![Effect {
                            input: Type::Existential(2),
                            output: Type::Number,
                        }],
                    },
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::Number),
                        effects: vec![Effect {
                            input: Type::Existential(2),
                            output: Type::Number,
                        }],
                    },
                ),
                (
                    3,
                    Type::Function {
                        parameter: Box::new(Type::Existential(2)),
                        body: Box::new(Type::Effectful {
                            ty: Box::new(Type::Number),
                            effects: vec![Effect {
                                input: Type::Existential(2),
                                output: Type::Number,
                            }],
                        }),
                    },
                ),
                (
                    4,
                    Type::Effectful {
                        ty: Box::new(Type::Number),
                        effects: vec![Effect {
                            input: Type::String,
                            output: Type::Number,
                        }],
                    }
                ),
            ],
        );
    }

    #[test]
    fn handle() {
        let (hirgen, expr) = parse_inner(
            r#"
            \ <x>, <y>, <z> ->
              #3 <x> => <y>
                $ ! 1 => <'string> ~
                #1 ! <y> => <z> ~
              #2 <\y -> z> ! <x> => <y>
        "#,
        );
        let ctx = Ctx::default();
        let (ctx, _ty) = ctx.synth(&expr).unwrap();

        dbg!(&ctx.logs);

        // x: 1, y: 5, z: 9
        assert_eq!(
            get_types(&hirgen, &ctx),
            vec![
                (
                    1,
                    Type::Effectful {
                        ty: Box::new(Type::Existential(9)),
                        effects: vec![Effect {
                            input: Type::Existential(5),
                            output: Type::Existential(9),
                        }],
                    },
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::Existential(9)),
                        effects: vec![Effect {
                            input: Type::Existential(1),
                            output: Type::Existential(5),
                        }],
                    },
                ),
                (
                    3,
                    Type::Effectful {
                        ty: Box::new(Type::Existential(9)),
                        effects: vec![Effect {
                            input: Type::Number,
                            output: Type::String,
                        }],
                    },
                ),
            ],
        );
    }

    #[test]
    fn label() {
        let expr = parse(
            r#"
            ^^^1: <@labeled 'number>: <'number>: <@labeled 'number>
        "#,
        );
        assert_eq!(
            synth(expr),
            Ok(Type::Label {
                label: "labeled".into(),
                item: Box::new(Type::Number),
            })
        );
    }

    #[test]
    fn instantiate_label() {
        let expr = parse(
            r#"
            \ <x> -> ^<x>: <@labeled 'number>
        "#,
        );
        assert_eq!(
            synth(expr),
            Ok(Type::Function {
                parameter: Box::new(Type::Label {
                    label: "labeled".into(),
                    item: Box::new(Type::Number),
                }),
                body: Box::new(Type::Label {
                    label: "labeled".into(),
                    item: Box::new(Type::Number),
                })
            })
        );
    }

    #[test]
    fn brand_supertype() {
        let expr = parse(
            r#"
            'brand brand
            ^1: <@brand 'number>
        "#,
        );
        assert_eq!(
            synth(expr),
            Err(TypeError::NotSubtype {
                sub: Type::Number,
                ty: Type::Brand {
                    brand: "brand".into(),
                    item: Box::new(Type::Number),
                },
            })
        );
    }

    #[test]
    fn brand_subtype() {
        let expr = parse(
            r#"
            'brand brand
            ^<@brand 'number>: <'number>
        "#,
        );
        assert_eq!(synth(expr), Ok(Type::Number));
    }

    #[test]
    fn infer() {
        let (hirgen, expr) = parse_inner(
            r#"
            ^<\ #1 _ -> #2 _> "a": <'number>
            "#,
        );
        let ctx = Ctx::default();
        let (ctx, ty) = ctx.synth(&expr).unwrap();

        assert_eq!(ty, Type::Number);
        assert_eq!(
            get_types(&hirgen, &ctx),
            vec![(1, Type::String,), (2, Type::Number,)]
        );
    }

    #[test]
    fn test_match() {
        let (hirgen, expr) = parse_inner(
            r#"
            \ <x> ->
              #2 + #1 <x> ~
                <'number> -> ^1: <@a 'number>,
                <'string> -> ^2: <@b 'number>.
            "#,
        );
        let ctx = Ctx::default();
        let (ctx, _ty) = ctx.synth(&expr).unwrap();

        assert_eq!(
            get_types(&hirgen, &ctx),
            vec![
                (1, Type::Sum(vec![Type::Number, Type::String])),
                (
                    2,
                    Type::Sum(vec![
                        Type::Label {
                            label: "a".into(),
                            item: Box::new(Type::Number)
                        },
                        Type::Label {
                            label: "b".into(),
                            item: Box::new(Type::Number)
                        }
                    ])
                )
            ]
        );
    }

    // TODO:
    // Priority labels in function application
    // Priority labels in product and sum
}
