use types::Id;

use crate::{
    ty::{Type, TypeVisitor},
    ctx::Ctx,
};

pub(crate) struct WellFormed<'a> {
    pub ctx: &'a Ctx,
    pub well_formed: bool,
}

impl<'a> TypeVisitor for WellFormed<'a> {
    fn visit_existential(&mut self, id: &Id) {
        self.well_formed = self.ctx.has_existential(id) || self.ctx.get_solved(id).is_some();
    }

    fn visit_variable(&mut self, id: &Id) {
        self.well_formed = self.ctx.has_variable(id);
    }

    fn visit(&mut self, ty: &Type) {
		if self.well_formed {
			self.visit_inner(ty);
		}
	}
}
