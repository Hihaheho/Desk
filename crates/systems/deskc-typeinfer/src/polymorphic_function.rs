use std::collections::HashMap;

use hir::{expr::Expr, meta::WithMeta};

use crate::{
    ctx::{Ctx, Id},
    ty::{Type, TypeVisitorMut},
};

impl Ctx {
    pub(crate) fn make_polymorphic(self, expr: &WithMeta<Expr>, ty: Type) -> Self {
        if let Type::Function { .. } = ty {
            let ty = self.to_polymorphic_function(ty);
            self.replace_type(expr, ty)
        } else {
            self
        }
    }

    fn replace_type(self, expr: &WithMeta<Expr>, ty: Type) -> Self {
        *self.ir_types.borrow_mut().get_mut(&expr.meta.id).unwrap() = ty;
        self
    }

    fn to_polymorphic_function(&self, mut ty: Type) -> Type {
        let mut visitor = Visitor {
            ctx: self,
            ids: Default::default(),
        };
        visitor.visit(&mut ty);
        let mut ids: Vec<_> = visitor.ids.values().collect();
        ids.sort();
        ids.into_iter().rev().fold(ty, |ty, id| Type::ForAll {
            variable: *id,
            bound: None,
            body: Box::new(ty),
        })
    }
}

struct Visitor<'a> {
    ctx: &'a Ctx,
    ids: HashMap<Id, Id>,
}

impl<'a> TypeVisitorMut for Visitor<'a> {
    fn visit(&mut self, ty: &mut Type) {
        match ty {
            Type::Existential(id) => {
                if let Some(solved) = self.ctx.types.borrow().get(id) {
                    *ty = solved.clone();
                }
                if let Type::Existential(id) = ty {
                    *id = *self
                        .ids
                        .entry(*id)
                        .or_insert_with(|| self.ctx.fresh_existential());
                }
            }
            ty => self.visit_inner(ty),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function() {
        assert_eq!(
            Ctx::default().to_polymorphic_function(Type::Function {
                parameter: Box::new(Type::Real),
                body: Box::new(Type::Real)
            }),
            Type::Function {
                parameter: Box::new(Type::Real),
                body: Box::new(Type::Real)
            }
        );
    }

    #[test]
    fn function_existential() {
        assert_eq!(
            Ctx::default().to_polymorphic_function(Type::Function {
                parameter: Box::new(Type::Existential(1)),
                body: Box::new(Type::Existential(2))
            }),
            Type::ForAll {
                variable: 0,
                bound: None,
                body: Box::new(Type::ForAll {
                    variable: 1,
                    bound: None,
                    body: Box::new(Type::Function {
                        parameter: Box::new(Type::Existential(0)),
                        body: Box::new(Type::Existential(1))
                    })
                })
            }
        );
    }

    #[test]
    fn function_existential_solved() {
        let ctx = Ctx::default();
        ctx.store_solved_type_and_effects(1, Type::Real, Default::default());
        ctx.store_solved_type_and_effects(2, Type::String, Default::default());
        assert_eq!(
            ctx.to_polymorphic_function(Type::Function {
                parameter: Box::new(Type::Existential(1)),
                body: Box::new(Type::Existential(2))
            }),
            Type::Function {
                parameter: Box::new(Type::Real),
                body: Box::new(Type::String)
            }
        );
    }

    #[test]
    fn function_existential_solved_but_existential() {
        let ctx = Ctx::default();
        ctx.store_solved_type_and_effects(1, Type::Existential(3), Default::default());
        ctx.store_solved_type_and_effects(2, Type::Existential(4), Default::default());
        assert_eq!(
            ctx.to_polymorphic_function(Type::Function {
                parameter: Box::new(Type::Existential(1)),
                body: Box::new(Type::Existential(2))
            }),
            Type::ForAll {
                variable: 0,
                bound: None,
                body: Box::new(Type::ForAll {
                    variable: 1,
                    bound: None,
                    body: Box::new(Type::Function {
                        parameter: Box::new(Type::Existential(0)),
                        body: Box::new(Type::Existential(1))
                    })
                })
            }
        );
    }
}
