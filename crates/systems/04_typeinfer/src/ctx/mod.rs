mod apply;
mod check;
mod instantiate_subtype;
mod instantiate_supertype;
mod subtype;
mod synth;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use hir::meta::WithMeta;
use types::{IdGen, Types};

use crate::{
    error::TypeError,
    from_hir_type, into_type,
    substitute_from_ctx::SubstituteFromCtx,
    ty::{Effect, Type, TypeVisitor, TypeVisitorMut},
    utils::with_effects,
    well_formed::WellFormed,
    with_effects::WithEffects,
};

pub type Id = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Log {
    Variable(Id),
    TypedVariable(Id, Type),
    Existential(Id),
    Solved(Id, Type),
    Marker(Id),
    Effect(Effect),
}

#[must_use]
#[derive(Default, Debug, Clone)]
pub struct Ctx {
    pub(crate) id_gen: Rc<RefCell<IdGen>>,
    pub(crate) logs: RefCell<Vec<Log>>,
    // Result of type inference
    pub(crate) types: Rc<RefCell<HashMap<Id, Type>>>,
    // a stack; continue's input of current context
    pub(crate) continue_input: RefCell<Vec<Type>>,
    // a stack; continue's output of current context
    pub(crate) continue_output: RefCell<Vec<Type>>,
}

impl Ctx {
    pub fn next_id(&self) -> Id {
        self.id_gen.borrow_mut().next_id()
    }
    fn empty(&self) -> Self {
        Self {
            id_gen: self.id_gen.clone(),
            logs: Default::default(),
            types: self.types.clone(),
            continue_input: Default::default(),
            continue_output: Default::default(),
        }
    }

    // The type should be substituted with ctx.
    fn store_type_and_effects<T>(&self, expr: &WithMeta<T>, ty: Type, effects: Vec<Effect>) {
        self.store_type(expr.meta.id, with_effects(ty, effects));
    }

    fn store_type(&self, id: Id, ty: Type) {
        if let Type::Infer(_) = ty {
            // infer should not be registered
            return;
        }
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
        self.store_type_and_effects(&hir_ty, ty.clone(), vec![]);
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

    fn begin_scope(&self) -> Id {
        let id = self.fresh_existential();
        self.logs.borrow_mut().push(Log::Marker(id));
        id
    }

    fn end_scope(&self, scope: Id) -> Vec<Effect> {
        let index = self.index(&Log::Marker(scope)).expect("scope should exist");
        let mut effects = Vec::new();
        let logs: Vec<_> = self.logs.borrow_mut().drain(index..).collect();
        for log in logs {
            match log {
                Log::Effect(effect) => effects.push(self.substitute_from_context_effect(&effect)),
                other => self.logs.borrow_mut().push(other),
            }
        }
        // Delete scope
        self.logs.borrow_mut().remove(index);
        effects
    }

    fn index(&self, log: &Log) -> Option<usize> {
        self.logs.borrow().iter().position(|x| x == log)
    }

    pub fn fresh_existential(&self) -> Id {
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

    fn truncate_from(&self, log: &Log) -> WithEffects<Ctx> {
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

    pub(crate) fn has_variable(&self, id: &Id) -> bool {
        self.logs
            .borrow()
            .iter()
            .any(|log| log == &Log::Variable(*id))
    }

    pub(crate) fn has_existential(&self, id: &Id) -> bool {
        self.logs
            .borrow()
            .iter()
            .any(|log| log == &Log::Existential(*id))
    }

    pub(crate) fn get_solved(&self, id: &Id) -> Option<Type> {
        self.logs.borrow().iter().find_map(|log| match log {
            Log::Solved(var, ty) if var == id => Some(ty.clone()),
            _ => None,
        })
    }

    pub(crate) fn get_typed_var(&self, id: &Id) -> Result<Type, TypeError> {
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

    fn is_well_formed(&self, ty: &Type) -> bool {
        let mut well_formed = WellFormed {
            ctx: self,
            well_formed: true,
        };
        well_formed.visit(ty);
        well_formed.well_formed
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

    pub fn substitute_from_context_effect(&self, Effect { input, output }: &Effect) -> Effect {
        Effect {
            input: self.substitute_from_ctx(input),
            output: self.substitute_from_ctx(output),
        }
    }

    pub fn add_effects<'a>(&self, effects: impl IntoIterator<Item = &'a Effect>) -> Ctx {
        effects
            .into_iter()
            .map(|effect| self.substitute_from_context_effect(effect))
            .fold(self.clone(), |ctx, effect| ctx.add(Log::Effect(effect)))
    }
}
