use crate::{
    ctx::Id,
    internal_type::{Type, TypeVisitor},
};

// existential type is occurs in the type
pub fn occurs_in(id: &Id, ty: &Type) -> bool {
    let mut occurs_in = OccursIn {
        id: *id,
        occurs_in: false,
    };
    occurs_in.visit(ty);
    occurs_in.occurs_in
}

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
