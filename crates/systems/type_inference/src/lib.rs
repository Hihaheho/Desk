mod error;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use error::TypeError;
use hir::{
    expr::{Expr, Literal},
    meta::WithMeta,
};
use types::Type;

pub type Id = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Log {
    Variable(Id),
    TypedVariable(Id, Type),
    Existential(Id),
    Solved(Id, Type),
    Marker(Id),
}

#[derive(Default, Debug, Clone)]
pub struct Ctx {
    next_id: Rc<RefCell<usize>>,
    logs: Vec<Log>,
    // Result of type inference
    expr_types: Rc<RefCell<HashMap<Id, Type>>>,
}

fn from_hir_type(ty: &hir::ty::Type) -> Type {
    use hir::ty::Type::*;
    match ty {
        Number => Type::Number,
        String => Type::String,
        Trait(types) => todo!(),
        Effectful { ty, effects } => todo!(),
        Infer => todo!(),
        This => todo!(),
        Product(types) => {
            Type::Product(types.into_iter().map(|t| from_hir_type(&t.value)).collect())
        }
        Sum(types) => Type::Sum(types.into_iter().map(|t| from_hir_type(&t.value)).collect()),
        Function { parameter, body } => Type::Function {
            parameter: Box::new(from_hir_type(&parameter.value)),
            body: Box::new(from_hir_type(&body.value)),
        },
        Array(ty) => Type::Array(Box::new(from_hir_type(&ty.value))),
        Set(ty) => Type::Set(Box::new(from_hir_type(&ty.value))),
        Let { definition, body } => todo!(),
        Variable(id) => Type::Variable(*id),
        BoundedVariable { bound, identifier } => todo!(),
    }
}

impl Ctx {
    fn empty(&self) -> Self {
        Self {
            next_id: self.next_id.clone(),
            logs: Vec::new(),
            expr_types: self.expr_types.clone(),
        }
    }

    fn store_type(&self, expr: &WithMeta<Expr>, ty: &Type) {
        if let Some(meta) = &expr.meta {
            self.expr_types
                .borrow_mut()
                .insert(meta.id, self.substitute_from_ctx(ty));
        }
    }

    fn index(&self, log: &Log) -> Option<usize> {
        self.logs.iter().position(|x| x == log)
    }

    fn fresh_existential(&self) -> Id {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() += 1;
        id
    }

    fn with_type(self, ty: Type) -> (Self, Type) {
        (self, ty)
    }

    fn add(&self, log: Log) -> Ctx {
        let mut cloned = self.clone();
        cloned.logs.push(log);
        cloned
    }

    fn insert_in_place(&self, log: &Log, logs: Vec<Log>) -> Ctx {
        let mut cloned = self.clone();
        let index = cloned.index(log).expect(&format!(
            "{:?}: log not found: {:?} to be replaced {:?}",
            self.logs, log, logs
        ));
        cloned.logs.splice(index..=index, logs);
        cloned
    }

    fn truncate_from(&self, log: &Log) -> Ctx {
        let mut cloned = self.clone();
        cloned.logs.truncate(self.index(log).expect(&format!(
            "{:?}: log not found: {:?} to be truncated",
            self.logs, log
        )));
        cloned
    }

    fn has_variable(&self, id: &Id) -> bool {
        self.logs.iter().any(|log| log == &Log::Variable(*id))
    }

    fn has_existential(&self, id: &Id) -> bool {
        self.logs.iter().any(|log| log == &Log::Existential(*id))
    }

    fn get_solved(&self, id: &Id) -> Option<Type> {
        self.logs.iter().find_map(|log| match log {
            Log::Solved(var, ty) if var == id => Some(ty.clone()),
            _ => None,
        })
    }

    fn get_type(&self, id: &Id) -> Result<Type, TypeError> {
        self.logs
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

    fn check(&self, expr: &WithMeta<Expr>, ty: &Type) -> Result<Ctx, TypeError> {
        let ctx = match (&expr.value, ty) {
            (Expr::Literal(Literal::Int(_)), Type::Number) => self.clone(),
            (Expr::Literal(Literal::Float(_)), Type::Number) => self.clone(),
            (Expr::Literal(Literal::Rational(_, _)), Type::Number) => self.clone(),
            (Expr::Literal(Literal::String(_)), Type::String) => self.clone(),
            (Expr::Literal(Literal::Uuid(_)), Type::String) => self.clone(),
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
        ctx.store_type(expr, ty);
        Ok(ctx)
    }

    pub fn synth(&self, expr: &WithMeta<Expr>) -> Result<(Ctx, Type), TypeError> {
        let (ctx, ty) = match &expr.value {
            Expr::Literal(Literal::Int(_)) => (self.clone(), Type::Number),
            Expr::Literal(Literal::Float(_)) => (self.clone(), Type::Number),
            Expr::Literal(Literal::Rational(_, _)) => (self.clone(), Type::Number),
            Expr::Literal(Literal::String(_)) => (self.clone(), Type::String),
            Expr::Literal(Literal::Uuid(_)) => (self.clone(), Type::String),
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
                    self.synth(&expression)?
                }
            }
            Expr::Perform { input, output } => todo!(),
            Expr::Handle {
                input,
                output,
                handler,
                expr,
            } => todo!(),
            Expr::Apply {
                function,
                arguments,
            } => {
                if arguments.is_empty() {
                    // Reference
                    let fun = from_hir_type(&function.value);
                    if let Type::Variable(id) = fun {
                        self.clone().with_type(self.get_type(&id)?)
                    } else {
                        self.clone().with_type(fun)
                    }
                } else {
                    // Normal application
                    let fun = match from_hir_type(&function.value) {
                        Type::Variable(var) => self.get_type(&var)?,
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
                let ty = from_hir_type(&ty.value);
                self.check(&expr, &ty)?.with_type(ty)
            }
            Expr::Hole => todo!(),
            Expr::Function { parameter, body } => {
                if let Type::Variable(id) = from_hir_type(&parameter.value) {
                    let a = self.fresh_existential();
                    let b = self.fresh_existential();
                    self.add(Log::Existential(a))
                        .add(Log::Existential(b))
                        .add(Log::TypedVariable(id, Type::Existential(a)))
                        .check(&body, &Type::Existential(b))?
                        .truncate_from(&Log::TypedVariable(id, Type::Existential(a)))
                        .with_type(Type::Function {
                            parameter: Box::new(Type::Existential(a)),
                            body: Box::new(Type::Existential(b)),
                        })
                } else {
                    let a = self.fresh_existential();
                    self.add(Log::Existential(a))
                        .check(&body, &Type::Existential(a))?
                        .with_type(Type::Function {
                            parameter: Box::new(from_hir_type(&parameter.value)),
                            body: Box::new(Type::Existential(a)),
                        })
                }
            }
            Expr::Array(_) => todo!(),
            Expr::Set(_) => todo!(),
        };
        ctx.store_type(expr, &ty);
        Ok((ctx, ty))
    }

    fn apply(&self, ty: &Type, expr: &WithMeta<Expr>) -> Result<(Ctx, Type), TypeError> {
        let ret = match ty {
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
                (delta, ty)
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
        match ty {
            Type::Number => true,
            Type::String => true,
            Type::Function { parameter, body } => {
                self.is_well_formed(parameter) && self.is_well_formed(body)
            }
            Type::Product(types) => types.iter().all(|ty| self.is_well_formed(ty)),
            Type::Sum(types) => types.iter().all(|ty| self.is_well_formed(ty)),
            Type::Array(ty) => self.is_well_formed(ty),
            Type::Set(ty) => self.is_well_formed(ty),
            Type::ForAll { variable, body } => {
                self.add(Log::Variable(*variable)).is_well_formed(body)
            }
            Type::Variable(id) => self.has_variable(id),
            Type::Existential(id) => self.has_existential(id) || self.get_solved(id).is_some(),
        }
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
            }
            (sub, Type::ForAll { variable, body }) => self
                .add(Log::Variable(*variable))
                .subtype(sub, body)?
                .truncate_from(&Log::Variable(*variable)),
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
            (_, _) => Err(TypeError::NotSubtype {
                sub: sub.clone(),
                ty: ty.clone(),
            })?,
        };
        Ok(ctx)
    }

    fn instantiate_subtype(&self, id: &Id, sup: &Type) -> Result<Ctx, TypeError> {
        // In here, we can assume the context contains the existential type.
        if is_monotype(sup)
            && self
                .truncate_from(&Log::Existential(*id))
                .is_well_formed(sup)
        {
            return Ok(
                self.insert_in_place(&Log::Existential(*id), vec![Log::Solved(*id, sup.clone())])
            );
        }
        let ctx = match sup {
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
                .truncate_from(&Log::Variable(*variable)),
            Type::Existential(var) => self.insert_in_place(
                &Log::Existential(*var),
                vec![Log::Solved(*var, Type::Existential(*id))],
            ),
            Type::Product(types) => {
                self.instantiate_composite_type_vec(*id, types, Type::Product, |ctx, id, sup| {
                    ctx.instantiate_subtype(id, sup)
                })?
            }
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
        };
        Ok(ctx)
    }

    fn instantiate_supertype(&self, sub: &Type, id: &Id) -> Result<Ctx, TypeError> {
        // In here, we can assume the context contains the existential type.
        if is_monotype(sub)
            && self
                .truncate_from(&Log::Existential(*id))
                .is_well_formed(sub)
        {
            return Ok(
                self.insert_in_place(&Log::Existential(*id), vec![Log::Solved(*id, sub.clone())])
            );
        }
        let ctx = match sub {
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
                .truncate_from(&Log::Marker(*variable)),
            Type::Existential(var) => self.insert_in_place(
                &Log::Existential(*var),
                vec![Log::Solved(*var, Type::Existential(*id))],
            ),
            Type::Product(types) => {
                self.instantiate_composite_type_vec(*id, types, Type::Product, |ctx, id, sub| {
                    ctx.instantiate_supertype(sub, id)
                })?
            }
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
        };
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
        match a {
            Type::Number => Type::Number,
            Type::String => Type::String,
            Type::Product(types) => Type::Product(
                types
                    .iter()
                    .map(|ty| self.substitute_from_ctx(ty))
                    .collect(),
            ),
            Type::Sum(types) => Type::Sum(
                types
                    .iter()
                    .map(|ty| self.substitute_from_ctx(ty))
                    .collect(),
            ),
            Type::Function { parameter, body } => Type::Function {
                parameter: Box::new(self.substitute_from_ctx(parameter)),
                body: Box::new(self.substitute_from_ctx(body)),
            },
            Type::Array(ty) => Type::Array(Box::new(self.substitute_from_ctx(ty))),
            Type::Set(ty) => Type::Set(Box::new(self.substitute_from_ctx(ty))),
            Type::Variable(id) => self.get_type(id).unwrap_or(a.clone()),
            Type::ForAll { variable, body } => Type::ForAll {
                variable: *variable,
                body: Box::new(self.substitute_from_ctx(body)),
            },
            Type::Existential(id) => self.get_solved(id).unwrap_or(a.clone()),
        }
    }
}

fn substitute(to: &Type, id: &Id, by: &Type) -> Type {
    let sub_if = |pred: bool| -> Type {
        if pred {
            by.clone()
        } else {
            to.clone()
        }
    };
    match to {
        Type::Number => Type::Number,
        Type::String => Type::String,
        Type::Product(types) => {
            Type::Product(types.iter().map(|ty| substitute(ty, id, by)).collect())
        }
        Type::Sum(types) => Type::Sum(types.iter().map(|ty| substitute(ty, id, by)).collect()),
        Type::Function { parameter, body } => Type::Function {
            parameter: Box::new(substitute(parameter, id, by)),
            body: Box::new(substitute(body, id, by)),
        },
        Type::Array(ty) => Type::Array(Box::new(substitute(ty, id, by))),
        Type::Set(ty) => Type::Set(Box::new(substitute(ty, id, by))),
        Type::Variable(var) => sub_if(*var == *id),
        Type::ForAll { variable, body } => Type::ForAll {
            variable: *variable,
            body: Box::new(substitute(body, id, by)),
        },
        Type::Existential(var) => sub_if(*var == *id),
    }
}

// existential type is occurs in the type
fn occurs_in(id: &Id, ty: &Type) -> bool {
    match ty {
        Type::Variable(_) => false,
        Type::Number => false,
        Type::String => false,
        Type::Product(types) => types.iter().any(|ty| occurs_in(id, ty)),
        Type::Sum(types) => types.iter().any(|ty| occurs_in(id, ty)),
        Type::Function { parameter, body } => occurs_in(id, parameter) || occurs_in(id, body),
        Type::Array(ty) => occurs_in(id, ty),
        Type::Set(ty) => occurs_in(id, ty),
        Type::ForAll { variable, body } => variable == id || occurs_in(id, body),
        Type::Existential(ty_id) => ty_id == id,
    }
}

fn is_monotype(ty: &Type) -> bool {
    match ty {
        Type::Number => true,
        Type::String => true,
        Type::Product(types) => types.iter().all(|ty| is_monotype(ty)),
        Type::Sum(types) => types.iter().all(|ty| is_monotype(ty)),
        Type::Function { parameter, body } => is_monotype(parameter) && is_monotype(body),
        Type::Array(ty) => is_monotype(&*ty),
        Type::Set(ty) => is_monotype(&*ty),
        Type::Variable(_) => true,
        Type::ForAll { .. } => false,
        Type::Existential(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use hir::meta::no_meta;
    use hirgen::HirGen;

    use super::*;

    fn synth(expr: WithMeta<Expr>) -> Result<Type, TypeError> {
        let (ctx, ty) = Ctx::default().synth(&expr)?;
        Ok(ctx.substitute_from_ctx(&ty))
    }

    fn parse(input: &str) -> WithMeta<Expr> {
        parse_inner(input).1
    }

    fn parse_inner(input: &str) -> (HirGen, WithMeta<Expr>) {
        use chumsky::prelude::end;
        use chumsky::{Parser, Stream};
        let expr = parser::expr::parser()
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
        let expr = gen.gen(expr).unwrap();
        (gen, expr)
    }

    fn assert_types(hirgen: &HirGen, ctx: &Ctx, types: Vec<(Expr, Type)>) {
        let attrs: HashMap<String, Id> = hirgen
            .expr_attrs
            .borrow()
            .iter()
            .map(|(id, attr)| (format!("{:?}", attr), id.clone()))
            .collect();
        for (expr, ty) in types {
            assert_eq!(
                ctx.expr_types
                    .borrow()
                    .get(attrs.get(&format!("{:?}", expr)).unwrap()),
                Some(&ty),
                "attr is {:?}",
                expr
            );
        }
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
                parameter: Box::new(Type::Existential(0)),
                body: Box::new(Type::Existential(0)),
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
            #4 <id> #5 1
        "#,
        );
        let ctx = Ctx::default();
        let (ctx, _ty) = ctx.synth(&expr).unwrap();

        assert_types(
            &hirgen,
            &ctx,
            vec![
                (Expr::Literal(Literal::Int(1)), Type::Number),
                (
                    Expr::Literal(Literal::Int(2)),
                    Type::Function {
                        parameter: Box::new(Type::Existential(0)),
                        body: Box::new(Type::Existential(0)),
                    },
                ),
                (Expr::Literal(Literal::Int(3)), Type::Existential(0)),
                (Expr::Literal(Literal::Int(4)), Type::Number),
                (Expr::Literal(Literal::Int(5)), Type::Number),
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

        assert_types(
            &hirgen,
            &ctx,
            vec![
                (
                    Expr::Literal(Literal::Int(1)),
                    Type::Function {
                        parameter: Box::new(Type::Sum(vec![Type::Number, Type::Product(vec![])])),
                        body: Box::new(Type::Number),
                    },
                ),
                (
                    Expr::Literal(Literal::Int(2)),
                    Type::Product(vec![Type::Number, Type::String]),
                ),
                (Expr::Literal(Literal::Int(3)), Type::Number),
            ],
        );
    }
}
