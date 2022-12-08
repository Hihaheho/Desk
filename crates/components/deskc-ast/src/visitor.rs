use dson::Dson;
use ids::{CardId, LinkName};

use crate::{
    expr::{Expr, Handler, Literal, MapElem, MatchCase},
    span::WithSpan,
    ty::{Effect, EffectExpr, Function, Type},
};

pub trait ExprVisitorMut {
    fn visit_expr(&mut self, expr: &mut WithSpan<Expr>) {
        self.super_visit_expr(expr);
    }
    fn super_visit_expr(&mut self, expr: &mut WithSpan<Expr>) {
        match &mut expr.value {
            Expr::Literal(literal) => self.visit_literal(literal),
            Expr::Do { stmt, expr } => self.visit_do(stmt, expr),
            Expr::Let { definition, body } => self.visit_let(definition, body),
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
            Expr::Function { parameter, body } => self.visit_function(parameter, body),
            Expr::Vector(exprs) => self.visit_vector(exprs),
            Expr::Map(exprs) => self.visit_map(exprs),
            Expr::Attributed { attr, item } => self.visit_attribute(attr, item),
            Expr::Brand { brand, item } => self.visit_brand(brand, item),
            Expr::Label { label, item } => self.visit_label(label, item),
            Expr::NewType { ident, ty, expr } => self.visit_new_type(ident, ty, expr),
            Expr::Comment { text, item } => self.visit_comment(text, item),
            Expr::Card { id, item, next } => self.visit_card(id, item, next),
        }
    }
    fn visit_literal(&mut self, _literal: &mut Literal) {}
    fn visit_do(&mut self, stmt: &mut WithSpan<Expr>, expr: &mut WithSpan<Expr>) {
        self.visit_expr(stmt);
        self.visit_expr(expr);
    }
    fn visit_let(&mut self, definition: &mut WithSpan<Expr>, body: &mut WithSpan<Expr>) {
        self.visit_expr(definition);
        self.visit_expr(body);
    }
    fn visit_perform(&mut self, input: &mut WithSpan<Expr>, output: &mut WithSpan<Type>) {
        self.visit_expr(input);
        self.visit_type(output);
    }
    fn visit_continue(&mut self, input: &mut WithSpan<Expr>, output: &mut WithSpan<Type>) {
        self.visit_expr(input);
        self.visit_type(output);
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
    fn visit_function(&mut self, parameter: &mut WithSpan<Type>, body: &mut WithSpan<Expr>) {
        self.visit_type(parameter);
        self.visit_expr(body);
    }
    fn visit_vector(&mut self, exprs: &mut Vec<WithSpan<Expr>>) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }
    fn visit_map(&mut self, exprs: &mut Vec<MapElem>) {
        for elem in exprs {
            self.visit_expr(&mut elem.key);
            self.visit_expr(&mut elem.value);
        }
    }
    fn visit_attribute(&mut self, _attr: &mut Dson, item: &mut WithSpan<Expr>) {
        self.visit_expr(item);
    }
    fn visit_brand(&mut self, _brand: &mut Dson, item: &mut WithSpan<Expr>) {
        self.visit_expr(item);
    }
    fn visit_label(&mut self, _label: &mut Dson, item: &mut WithSpan<Expr>) {
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
    fn visit_comment(&mut self, _text: &mut String, item: &mut WithSpan<Expr>) {
        self.visit_expr(item);
    }
    fn visit_card(
        &mut self,
        _id: &mut CardId,
        item: &mut WithSpan<Expr>,
        next: &mut WithSpan<Expr>,
    ) {
        self.visit_expr(item);
        self.visit_expr(next);
    }
    fn visit_type(&mut self, _ty: &mut WithSpan<Type>) {}
}

pub trait TypeVisitorMut {
    fn visit_type(&mut self, ty: &mut WithSpan<Type>) {
        self.super_visit_type(ty);
    }
    fn super_visit_type(&mut self, ty: &mut WithSpan<Type>) {
        match &mut ty.value {
            Type::Labeled { brand, item } => self.visit_brand(brand, item),
            Type::Real => self.visit_real(),
            Type::Rational => self.visit_rational(),
            Type::Integer => self.visit_integer(),
            Type::String => self.visit_string(),
            Type::Trait(types) => self.visit_trait(types),
            Type::Effectful { ty, effects } => self.visit_effectful(ty, effects),
            Type::Infer => self.visit_infer(),
            Type::This => self.visit_this(),
            Type::Product(types) => self.visit_product(types),
            Type::Sum(types) => self.visit_sum(types),
            Type::Function(function) => {
                self.visit_function(&mut function.parameter, &mut function.body)
            }
            Type::Vector(ty) => self.visit_vector(ty),
            Type::Map { key, value } => self.visit_map(key, value),
            Type::Let {
                variable,
                definition,
                body,
            } => self.visit_let(variable, definition, body),
            Type::Variable(ident) => self.visit_variable(ident),
            Type::Attributed { attr, ty } => self.visit_attribute(attr, ty),
            Type::Comment { text, item } => self.visit_comment(text, item),
            Type::Forall {
                variable,
                bound,
                body,
            } => self.visit_all(variable, bound, body),
            Type::Exists {
                variable,
                bound,
                body,
            } => self.visit_exists(variable, bound, body),
        }
    }
    fn visit_brand(&mut self, _brand: &mut Dson, item: &mut WithSpan<Type>) {
        self.visit_type(item);
    }
    fn visit_real(&mut self) {}
    fn visit_integer(&mut self) {}
    fn visit_rational(&mut self) {}
    fn visit_string(&mut self) {}
    fn visit_trait(&mut self, types: &mut [WithSpan<Function>]) {
        for ty in types {
            self.visit_type(&mut ty.value.parameter);
            self.visit_type(&mut ty.value.body);
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
    fn visit_function(&mut self, parameter: &mut WithSpan<Type>, body: &mut WithSpan<Type>) {
        self.visit_type(parameter);
        self.visit_type(body);
    }
    fn visit_vector(&mut self, ty: &mut WithSpan<Type>) {
        self.visit_type(ty);
    }
    fn visit_map(&mut self, key: &mut WithSpan<Type>, value: &mut WithSpan<Type>) {
        self.visit_type(key);
        self.visit_type(value);
    }
    fn visit_let(
        &mut self,
        _variable: &mut String,
        definition: &mut WithSpan<Type>,
        body: &mut WithSpan<Type>,
    ) {
        self.visit_type(definition);
        self.visit_type(body);
    }
    fn visit_variable(&mut self, _ident: &mut String) {}
    fn visit_attribute(&mut self, _attr: &mut Dson, item: &mut WithSpan<Type>) {
        self.visit_type(item);
    }
    fn visit_comment(&mut self, _text: &mut String, item: &mut WithSpan<Type>) {
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
    fn visit_all(
        &mut self,
        _variable: &mut String,
        bound: &mut Option<Box<WithSpan<Type>>>,
        body: &mut WithSpan<Type>,
    ) {
        if let Some(bound) = bound {
            self.visit_type(bound);
        }
        self.visit_type(body);
    }
    fn visit_exists(
        &mut self,
        _variable: &mut String,
        bound: &mut Option<Box<WithSpan<Type>>>,
        body: &mut WithSpan<Type>,
    ) {
        if let Some(bound) = bound {
            self.visit_type(bound);
        }
        self.visit_type(body);
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
