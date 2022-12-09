use ids::NodeId;

use crate::{
    expr::Expr,
    meta::WithMeta,
    visitor::{HirVisitor, TypeVisitor},
};

impl WithMeta<Expr> {
    pub fn get_expr_ids(&self) -> impl Iterator<Item = NodeId> {
        struct ExprIds {
            ids: Vec<NodeId>,
        }
        impl HirVisitor for ExprIds {
            fn visit_expr(&mut self, expr: &WithMeta<Expr>) {
                self.ids.push(expr.meta.id.clone());
                self.super_visit_expr(expr);
            }
            fn visit_type(&mut self, ty: &WithMeta<crate::ty::Type>) {
                TypeVisitor::visit_type(self, ty);
            }
        }
        impl TypeVisitor for ExprIds {
            fn visit_type(&mut self, ty: &WithMeta<crate::ty::Type>) {
                self.ids.push(ty.meta.id.clone());
                self.super_visit_type(ty);
            }
        }
        let mut ids = ExprIds { ids: Vec::new() };
        ids.visit_expr(self);
        ids.ids.into_iter()
    }
}
