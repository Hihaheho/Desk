use std::collections::HashMap;

use crate::{
    ctx::{Ctx, Id},
    ty::{Type, TypeVisitorMut},
};

impl Ctx {
    pub(crate) fn make_polymorphic(&self, mut ty: Type) -> Type {
        if let Type::Function { .. } = ty {
            let mut visitor = Visitor {
                ctx: self,
                ids: Default::default(),
            };
            visitor.visit(&mut ty);
            let mut ids: Vec<_> = visitor.ids.values().collect();
            ids.sort();
            ids.into_iter().rev().fold(ty, |ty, id| Type::ForAll {
                variable: *id,
                body: Box::new(ty),
            })
        } else {
            ty
        }
    }
}

struct Visitor<'a> {
    ctx: &'a Ctx,
    ids: HashMap<Id, Id>,
}

impl<'a> TypeVisitorMut for Visitor<'a> {
    fn visit_existential(&mut self, id: &mut Id) {
        *id = *self
            .ids
            .entry(*id)
            .or_insert_with(|| self.ctx.fresh_existential());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        assert_eq!(Ctx::default().make_polymorphic(Type::Number), Type::Number);
    }

    #[test]
    fn function() {
        assert_eq!(
            Ctx::default().make_polymorphic(Type::Function {
                parameter: Box::new(Type::Number),
                body: Box::new(Type::Number)
            }),
            Type::Function {
                parameter: Box::new(Type::Number),
                body: Box::new(Type::Number)
            }
        );
    }

    #[test]
    fn function_existential() {
        assert_eq!(
            Ctx::default().make_polymorphic(Type::Function {
                parameter: Box::new(Type::Existential(1)),
                body: Box::new(Type::Existential(2))
            }),
            Type::ForAll {
                variable: 0,
                body: Box::new(Type::ForAll {
                    variable: 1,
                    body: Box::new(Type::Function {
                        parameter: Box::new(Type::Existential(0)),
                        body: Box::new(Type::Existential(1))
                    })
                })
            }
        );
    }
}
