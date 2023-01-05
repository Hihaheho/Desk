use crate::{
    expr::Expr,
    meta::{Meta, WithMeta},
    ty::{Effect, Function, Type},
    visitor::{ExprVisitorMut, TypeVisitorMut},
};

// A helper function for testing to remove span information from AST.
pub fn replace_node_id_to_default(expr: &mut WithMeta<Expr>) {
    struct RemoveSpan;
    impl ExprVisitorMut for RemoveSpan {
        fn visit_meta(&mut self, meta: &mut Meta) {
            meta.id = Default::default();
        }
        fn visit_type(&mut self, ty: &mut WithMeta<Type>) {
            replace_node_id_to_default_ty(ty);
        }
    }
    RemoveSpan.visit_expr(expr)
}

// A helper function for testing to remove span information from AST.
pub fn replace_node_id_to_default_ty(ty: &mut WithMeta<Type>) {
    struct RemoveSpanTy;
    impl TypeVisitorMut for RemoveSpanTy {
        fn visit_type(&mut self, ty: &mut WithMeta<Type>) {
            ty.meta.id = Default::default();
            self.super_visit_type(ty);
        }
        fn visit_effect(&mut self, effect: &mut WithMeta<Effect>) {
            effect.meta.id = Default::default();
            self.visit_type(&mut effect.value.input);
            self.visit_type(&mut effect.value.output);
        }
        fn visit_trait(&mut self, types: &mut [WithMeta<Function>]) {
            for ty in types {
                ty.meta.id = Default::default();
                self.visit_type(&mut ty.value.parameter);
                self.visit_type(&mut ty.value.body);
            }
        }
        fn visit_effect_expr(&mut self, item: &mut WithMeta<crate::ty::EffectExpr>) {
            item.meta.id = Default::default();
            self.super_visit_effect_expr(item);
        }
    }
    RemoveSpanTy.visit_type(ty)
}
