use ids::LinkName;
use uuid::Uuid;

use crate::{
    expr::{Expr, Handler, Literal, MatchCase},
    span::WithSpan,
    ty::{CommentPosition, Effect, EffectExpr, Type},
};

pub trait ExprVisitorMut {
    fn visit_expr(&mut self, expr: &mut WithSpan<Expr>) {
        self.super_visit_expr(expr);
    }
    fn super_visit_expr(&mut self, expr: &mut WithSpan<Expr>) {
        match &mut expr.value {
            Expr::Literal(literal) => self.visit_literal(literal),
            Expr::Let {
                ty,
                definition,
                body,
            } => self.visit_let(ty, definition, body),
            Expr::Perform { input, output } => self.visit_perform(input, output),
            Expr::Continue { input, output } => self.visit_continue(input, output),
            Expr::Handle { expr, handlers } => self.visit_handle(expr, handlers),
            Expr::Apply {
                function,
                link_name,
                arguments,
            } => self.visit_apply(function, link_name, arguments),
            Expr::Product(exprs) => self.visit_product(exprs),
            Expr::Match { of, cases } => self.visit_match(of, cases),
            Expr::Typed { ty, item } => self.visit_typed(ty, item),
            Expr::Hole => self.visit_hole(),
            Expr::Function { parameters, body } => self.visit_function(parameters, body),
            Expr::Vector(exprs) => self.visit_vector(exprs),
            Expr::Set(exprs) => self.visit_set(exprs),
            Expr::Import { ty, uuid } => self.visit_import(ty, uuid),
            Expr::Export { ty } => self.visit_export(ty),
            Expr::Attribute { attr, item } => self.visit_attribute(attr, item),
            Expr::Brand { brands, item } => self.visit_brand(brands, item),
            Expr::Label { label, item } => self.visit_label(label, item),
            Expr::NewType { ident, ty, expr } => self.visit_new_type(ident, ty, expr),
            Expr::Comment {
                position,
                text,
                item,
            } => self.visit_comment(position, text, item),
            Expr::Card { uuid, item, next } => self.visit_card(uuid, item, next),
        }
    }
    fn visit_literal(&mut self, _literal: &mut Literal) {}
    fn visit_let(
        &mut self,
        ty: &mut WithSpan<Type>,
        definition: &mut WithSpan<Expr>,
        body: &mut WithSpan<Expr>,
    ) {
        self.visit_type(ty);
        self.visit_expr(definition);
        self.visit_expr(body);
    }
    fn visit_perform(&mut self, input: &mut WithSpan<Expr>, output: &mut WithSpan<Type>) {
        self.visit_expr(input);
        self.visit_type(output);
    }
    fn visit_continue(&mut self, input: &mut WithSpan<Expr>, output: &mut Option<WithSpan<Type>>) {
        self.visit_expr(input);
        if let Some(output) = output {
            self.visit_type(output);
        }
    }
    fn visit_handle(&mut self, expr: &mut WithSpan<Expr>, handlers: &mut Vec<Handler>) {
        self.visit_expr(expr);
        for handler in handlers {
            self.visit_handler(handler);
        }
    }
    fn visit_handler(&mut self, handler: &mut Handler) {
        self.visit_type(&mut handler.input);
        self.visit_type(&mut handler.output);
        self.visit_expr(&mut handler.handler);
    }
    fn visit_apply(
        &mut self,
        function: &mut WithSpan<Type>,
        _link_name: &mut LinkName,
        arguments: &mut Vec<WithSpan<Expr>>,
    ) {
        self.visit_type(function);
        for argument in arguments {
            self.visit_expr(argument);
        }
    }
    fn visit_product(&mut self, exprs: &mut Vec<WithSpan<Expr>>) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }
    fn visit_match(&mut self, of: &mut WithSpan<Expr>, cases: &mut Vec<MatchCase>) {
        self.visit_expr(of);
        for case in cases {
            self.visit_match_case(case);
        }
    }
    fn visit_match_case(&mut self, case: &mut MatchCase) {
        self.visit_type(&mut case.ty);
        self.visit_expr(&mut case.expr);
    }
    fn visit_typed(&mut self, ty: &mut WithSpan<Type>, item: &mut WithSpan<Expr>) {
        self.visit_type(ty);
        self.visit_expr(item);
    }
    fn visit_hole(&mut self) {}
    fn visit_function(&mut self, parameters: &mut Vec<WithSpan<Type>>, body: &mut WithSpan<Expr>) {
        for parameter in parameters {
            self.visit_type(parameter);
        }
        self.visit_expr(body);
    }
    fn visit_vector(&mut self, exprs: &mut Vec<WithSpan<Expr>>) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }
    fn visit_set(&mut self, exprs: &mut Vec<WithSpan<Expr>>) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }
    fn visit_import(&mut self, ty: &mut WithSpan<Type>, _uuid: &mut Option<Uuid>) {
        self.visit_type(ty);
    }
    fn visit_export(&mut self, ty: &mut WithSpan<Type>) {
        self.visit_type(ty);
    }
    fn visit_attribute(&mut self, attr: &mut WithSpan<Expr>, item: &mut WithSpan<Expr>) {
        self.visit_expr(attr);
        self.visit_expr(item);
    }
    fn visit_brand(&mut self, _brands: &mut Vec<String>, item: &mut WithSpan<Expr>) {
        self.visit_expr(item);
    }
    fn visit_label(&mut self, _label: &mut String, item: &mut WithSpan<Expr>) {
        self.visit_expr(item);
    }
    fn visit_new_type(
        &mut self,
        _ident: &mut String,
        ty: &mut WithSpan<Type>,
        expr: &mut WithSpan<Expr>,
    ) {
        self.visit_type(ty);
        self.visit_expr(expr);
    }
    fn visit_comment(
        &mut self,
        _position: &mut CommentPosition,
        _text: &mut String,
        item: &mut WithSpan<Expr>,
    ) {
        self.visit_expr(item);
    }
    fn visit_card(
        &mut self,
        _uuid: &mut Uuid,
        item: &mut WithSpan<Expr>,
        next: &mut Option<Box<WithSpan<Expr>>>,
    ) {
        self.visit_expr(item);
        if let Some(next) = next {
            self.visit_expr(next);
        }
    }
    fn visit_type(&mut self, _ty: &mut WithSpan<Type>) {}
}

pub trait TypeVisitorMut {
    fn visit_type(&mut self, ty: &mut WithSpan<Type>) {
        self.super_visit_type(ty);
    }
    fn super_visit_type(&mut self, ty: &mut WithSpan<Type>) {
        match &mut ty.value {
            Type::Brand { brand, item } => self.visit_brand(brand, item),
            Type::Number => self.visit_number(),
            Type::String => self.visit_string(),
            Type::Trait(types) => self.visit_trait(types),
            Type::Effectful { ty, effects } => self.visit_effectful(ty, effects),
            Type::Infer => self.visit_infer(),
            Type::This => self.visit_this(),
            Type::Product(types) => self.visit_product(types),
            Type::Sum(types) => self.visit_sum(types),
            Type::Function { parameters, body } => self.visit_function(parameters, body),
            Type::Vector(ty) => self.visit_vector(ty),
            Type::Set(ty) => self.visit_set(ty),
            Type::Let { variable, body } => self.visit_let(variable, body),
            Type::Variable(ident) => self.visit_variable(ident),
            Type::BoundedVariable { bound, identifier } => {
                self.visit_bounded_variable(bound, identifier)
            }
            Type::Attribute { attr, ty } => self.visit_attribute(attr, ty),
            Type::Comment {
                position,
                text,
                item,
            } => self.visit_comment(position, text, item),
        }
    }
    fn visit_brand(&mut self, _brand: &mut String, item: &mut WithSpan<Type>) {
        self.visit_type(item);
    }
    fn visit_number(&mut self) {}
    fn visit_string(&mut self) {}
    fn visit_trait(&mut self, types: &mut Vec<WithSpan<Type>>) {
        for ty in types {
            self.visit_type(ty);
        }
    }
    fn visit_effectful(&mut self, ty: &mut WithSpan<Type>, effects: &mut WithSpan<EffectExpr>) {
        self.visit_type(ty);
        self.visit_effect_expr(effects);
    }
    fn visit_infer(&mut self) {}
    fn visit_this(&mut self) {}
    fn visit_product(&mut self, types: &mut Vec<WithSpan<Type>>) {
        for ty in types {
            self.visit_type(ty);
        }
    }
    fn visit_sum(&mut self, types: &mut Vec<WithSpan<Type>>) {
        for ty in types {
            self.visit_type(ty);
        }
    }
    fn visit_function(&mut self, parameters: &mut Vec<WithSpan<Type>>, body: &mut WithSpan<Type>) {
        for parameter in parameters {
            self.visit_type(parameter);
        }
        self.visit_type(body);
    }
    fn visit_vector(&mut self, ty: &mut WithSpan<Type>) {
        self.visit_type(ty);
    }
    fn visit_set(&mut self, ty: &mut WithSpan<Type>) {
        self.visit_type(ty);
    }
    fn visit_import(&mut self, ty: &mut WithSpan<Type>, _uuid: &mut Option<Uuid>) {
        self.visit_type(ty);
    }
    fn visit_export(&mut self, ty: &mut WithSpan<Type>) {
        self.visit_type(ty);
    }
    fn visit_let(&mut self, _variable: &mut String, body: &mut WithSpan<Type>) {
        self.visit_type(body);
    }
    fn visit_variable(&mut self, _ident: &mut String) {}
    fn visit_bounded_variable(&mut self, _bound: &mut WithSpan<Type>, _identifier: &mut String) {}
    fn visit_attribute(&mut self, attr: &mut WithSpan<Expr>, item: &mut WithSpan<Type>) {
        self.visit_expr(attr);
        self.visit_type(item);
    }
    fn visit_comment(
        &mut self,
        _position: &mut CommentPosition,
        _text: &mut String,
        item: &mut WithSpan<Type>,
    ) {
        self.visit_type(item);
    }
    fn visit_expr(&mut self, _item: &mut WithSpan<Expr>) {}
    fn visit_effect_expr(&mut self, item: &mut WithSpan<EffectExpr>) {
        match &mut item.value {
            EffectExpr::Effects(effects) => {
                for effect in effects {
                    self.visit_effect(effect);
                }
            }
            EffectExpr::Add(exprs) => self.visit_effect_add(exprs),
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => self.visit_effect_sub(minuend, subtrahend),
            EffectExpr::Apply {
                function,
                arguments,
            } => self.visit_effect_apply(function, arguments),
        }
    }
    fn visit_effect(&mut self, _effect: &mut WithSpan<Effect>) {}
    fn visit_effect_add(&mut self, exprs: &mut Vec<WithSpan<EffectExpr>>) {
        for expr in exprs {
            self.visit_effect_expr(expr);
        }
    }
    fn visit_effect_sub(
        &mut self,
        minuend: &mut WithSpan<EffectExpr>,
        subtrahend: &mut WithSpan<EffectExpr>,
    ) {
        self.visit_effect_expr(minuend);
        self.visit_effect_expr(subtrahend);
    }
    fn visit_effect_apply(
        &mut self,
        function: &mut WithSpan<Type>,
        arguments: &mut Vec<WithSpan<Type>>,
    ) {
        self.visit_type(function);
        for argument in arguments {
            self.visit_type(argument);
        }
    }
}

pub fn remove_node_id(mut expr: WithSpan<Expr>) -> WithSpan<Expr> {
    struct RemoveNodeIdVisitor;
    impl ExprVisitorMut for RemoveNodeIdVisitor {
        fn visit_expr(&mut self, item: &mut WithSpan<Expr>) {
            item.id = Default::default();
            self.super_visit_expr(item);
        }
        fn visit_type(&mut self, item: &mut WithSpan<Type>) {
            struct RemoveNodeIdVisitorType;
            impl TypeVisitorMut for RemoveNodeIdVisitorType {
                fn visit_type(&mut self, item: &mut WithSpan<Type>) {
                    item.id = Default::default();
                    self.super_visit_type(item);
                }
            }
            RemoveNodeIdVisitorType.visit_type(item);
        }
    }
    let mut visitor = RemoveNodeIdVisitor;
    visitor.visit_expr(&mut expr);
    expr
}
