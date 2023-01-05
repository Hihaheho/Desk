use crate::{
    expr::Expr,
    span::WithSpan,
    ty::{Effect, Function, Type},
    visitor::{ExprVisitorMut, TypeVisitorMut},
};

pub fn remove_span(expr: &mut WithSpan<Expr>) {
    struct RemoveSpan;
    impl ExprVisitorMut for RemoveSpan {
        fn visit_span(&mut self, id: &mut ids::NodeId, span: &mut crate::span::Span) {
            *id = Default::default();
            *span = Default::default();
        }
        fn visit_type(&mut self, ty: &mut WithSpan<Type>) {
            remove_span_ty(ty);
        }
    }
    RemoveSpan.visit_expr(expr)
}

fn remove_span_ty(ty: &mut WithSpan<Type>) {
    struct RemoveSpanTy;
    impl TypeVisitorMut for RemoveSpanTy {
        fn visit_type(&mut self, ty: &mut WithSpan<Type>) {
            ty.id = Default::default();
            ty.span = Default::default();
            self.super_visit_type(ty);
        }
        fn visit_effect(&mut self, effect: &mut WithSpan<Effect>) {
            effect.id = Default::default();
            effect.span = Default::default();
            self.visit_type(&mut effect.value.input);
            self.visit_type(&mut effect.value.output);
        }
        fn visit_trait(&mut self, types: &mut [WithSpan<Function>]) {
            for ty in types {
                ty.id = Default::default();
                ty.span = Default::default();
                self.visit_type(&mut ty.value.parameter);
                self.visit_type(&mut ty.value.body);
            }
        }
        fn visit_effect_expr(&mut self, item: &mut WithSpan<crate::ty::EffectExpr>) {
            item.id = Default::default();
            item.span = Default::default();
            self.super_visit_effect_expr(item);
        }
    }
    RemoveSpanTy.visit_type(ty)
}
