use crate::{
    ty::{Type, TypeVisitorMut},
    ctx::Id,
};

pub fn substitute(to: &Type, id: &Id, by: &Type) -> Type {
    let mut substitute = Substitute {
        id: *id,
        ty: by.clone(),
    };
    let mut to = to.clone();
    substitute.visit(&mut to);
    to
}

pub(crate) struct Substitute {
    pub id: Id,
    pub ty: Type,
}

impl TypeVisitorMut for Substitute {
    fn visit(&mut self, ty: &mut Type) {
        match ty {
            Type::Existential(id) if *id == self.id => *ty = self.ty.clone(),
            Type::Variable(id) if *id == self.id => *ty = self.ty.clone(),
            ty => self.visit_inner(ty),
        }
    }
}
