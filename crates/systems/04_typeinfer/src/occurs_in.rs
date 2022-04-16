use crate::{
    ty::{Type, TypeVisitor},
    Id,
};

pub(crate) struct OccursIn {
    pub id: Id,
    pub occurs_in: bool,
}

impl TypeVisitor for OccursIn {
    fn visit_existential(&mut self, id: &Id) {
        if self.id == *id {
            self.occurs_in = true;
        }
    }
    fn visit(&mut self, ty: &Type) {
        // walk while not occurred
        if !self.occurs_in {
            self.visit_inner(ty)
        }
    }
}
