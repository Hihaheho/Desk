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
        *self.ir_types.borrow_mut().get_mut(&expr.id).unwrap() = ty;
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
    fn function() {
        assert_eq!(
            Ctx::default().to_polymorphic_function(Type::Function {
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
}
