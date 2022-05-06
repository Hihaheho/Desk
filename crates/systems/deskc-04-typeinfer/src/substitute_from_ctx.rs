use crate::{
    ty::{Type, TypeVisitorMut},
    Ctx,
};

pub(crate) struct SubstituteFromCtx<'a> {
    pub ctx: &'a Ctx,
}

impl<'a> TypeVisitorMut for SubstituteFromCtx<'a> {
    fn visit(&mut self, ty: &mut Type) {
        match ty {
            Type::Existential(id) => {
                if let Some(solved) = self.ctx.get_solved(id) {
                    *ty = solved;
                }
            }
            Type::Variable(id) => {
                if let Ok(typed) = self.ctx.get_typed_var(id) {
                    *ty = typed;
                }
            }
            Type::Infer(id) => {
                if let Some(typed) = self.ctx.inferred_types.borrow().get(id).cloned() {
                    *ty = typed;
                }
            }
            ty => self.visit_inner(ty),
        }
    }
}
