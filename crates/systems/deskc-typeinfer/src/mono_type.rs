use ids::NodeId;

use crate::{
    ctx::Id,
    internal_type::{Type, TypeVisitor},
};

pub fn is_monotype(ty: &Type) -> bool {
    let mut monotype = MonoType { is_monotype: true };
    monotype.visit(ty);
    monotype.is_monotype
}

pub(crate) struct MonoType {
    pub is_monotype: bool,
}

impl TypeVisitor for MonoType {
    fn visit_forall(&mut self, _variable: &Id, _bound: &Option<Box<Type>>, _body: &Type) {
        self.is_monotype = false;
    }
    fn visit_infer(&mut self, _id: &NodeId) {
        // TODO: this is too conservative, but we may not have a way to know in here
        self.is_monotype = false;
    }
    fn visit(&mut self, ty: &Type) {
        // walk while monotype
        if self.is_monotype {
            self.visit_inner(ty)
        }
    }
}
