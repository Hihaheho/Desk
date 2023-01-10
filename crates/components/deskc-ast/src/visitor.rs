use dson::Dson;
use ids::{CardId, LinkName};

use crate::{
    expr::{Expr, Handler, Literal, MapElem, MatchCase},
    meta::{Meta, WithMeta},
    ty::{Effect, EffectExpr, Function, Type},
};

// FIXME generate this with a derive macro
pub trait ExprVisitorMut {
    fn visit_expr(&mut self, expr: &mut WithMeta<Expr>) {
        self.super_visit_expr(expr);
    }
    fn super_visit_expr(&mut self, expr: &mut WithMeta<Expr>) {
        self.visit_meta(&mut expr.meta);
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
            Expr::DeclareBrand { brand, item } => self.visit_brand(brand, item),
            Expr::Label { label, item } => self.visit_label(label, item),
            Expr::NewType { ident, ty, expr } => self.visit_new_type(ident, ty, expr),
            Expr::Card { id, item, next } => self.visit_card(id, item, next),
        }
    }
    fn visit_literal(&mut self, _literal: &mut Literal) {}
    fn visit_do(&mut self, stmt: &mut WithMeta<Expr>, expr: &mut WithMeta<Expr>) {
        self.visit_expr(stmt);
        self.visit_expr(expr);
    }
    fn visit_let(&mut self, definition: &mut WithMeta<Expr>, body: &mut WithMeta<Expr>) {
        self.visit_expr(definition);
        self.visit_expr(body);
    }
    fn visit_perform(&mut self, input: &mut WithMeta<Expr>, output: &mut WithMeta<Type>) {
        self.visit_expr(input);
        self.visit_type(output);
    }
    fn visit_continue(&mut self, input: &mut WithMeta<Expr>, output: &mut WithMeta<Type>) {
        self.visit_expr(input);
        self.visit_type(output);
    }
    fn visit_handle(&mut self, expr: &mut WithMeta<Expr>, handlers: &mut Vec<WithMeta<Handler>>) {
        self.visit_expr(expr);
        for handler in handlers {
            self.visit_handler(handler);
        }
    }
    fn visit_handler(&mut self, handler: &mut WithMeta<Handler>) {
        self.visit_meta(&mut handler.meta);
        self.super_visit_handler(handler);
    }
    fn super_visit_handler(&mut self, handler: &mut WithMeta<Handler>) {
        self.visit_effect(&mut handler.value.effect);
        self.visit_expr(&mut handler.value.handler);
    }
    fn visit_effect(&mut self, effect: &mut WithMeta<Effect>) {
        self.visit_meta(&mut effect.meta);
        self.super_visit_effect(effect);
    }
    fn super_visit_effect(&mut self, effect: &mut WithMeta<Effect>) {
        self.visit_type(&mut effect.value.input);
        self.visit_type(&mut effect.value.output);
    }
    fn visit_apply(
        &mut self,
        function: &mut WithMeta<Type>,
        _link_name: &mut LinkName,
        arguments: &mut Vec<WithMeta<Expr>>,
    ) {
        self.visit_type(function);
        for argument in arguments {
            self.visit_expr(argument);
        }
    }
    fn visit_product(&mut self, exprs: &mut Vec<WithMeta<Expr>>) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }
    fn visit_match(&mut self, of: &mut WithMeta<Expr>, cases: &mut Vec<WithMeta<MatchCase>>) {
        self.visit_expr(of);
        for case in cases {
            self.visit_match_case(case);
        }
    }
    fn visit_match_case(&mut self, case: &mut WithMeta<MatchCase>) {
        self.visit_meta(&mut case.meta);
        self.super_visit_match_case(case);
    }
    fn super_visit_match_case(&mut self, case: &mut WithMeta<MatchCase>) {
        self.visit_type(&mut case.value.ty);
        self.visit_expr(&mut case.value.expr);
    }
    fn visit_typed(&mut self, ty: &mut WithMeta<Type>, item: &mut WithMeta<Expr>) {
        self.visit_type(ty);
        self.visit_expr(item);
    }
    fn visit_hole(&mut self) {}
    fn visit_function(&mut self, parameter: &mut WithMeta<Type>, body: &mut WithMeta<Expr>) {
        self.visit_type(parameter);
        self.visit_expr(body);
    }
    fn visit_vector(&mut self, exprs: &mut Vec<WithMeta<Expr>>) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }
    fn visit_map(&mut self, exprs: &mut Vec<WithMeta<MapElem>>) {
        for elem in exprs {
            self.visit_map_elem(elem);
        }
    }
    fn visit_map_elem(&mut self, elem: &mut WithMeta<MapElem>) {
        self.visit_meta(&mut elem.meta);
        self.super_visit_map_elem(elem);
    }
    fn super_visit_map_elem(&mut self, elem: &mut WithMeta<MapElem>) {
        self.visit_expr(&mut elem.value.key);
        self.visit_expr(&mut elem.value.value);
    }
    fn visit_attribute(&mut self, _attr: &mut Dson, item: &mut WithMeta<Expr>) {
        self.visit_expr(item);
    }
    fn visit_brand(&mut self, _brand: &mut Dson, item: &mut WithMeta<Expr>) {
        self.visit_expr(item);
    }
    fn visit_label(&mut self, _label: &mut Dson, item: &mut WithMeta<Expr>) {
        self.visit_expr(item);
    }
    fn visit_new_type(
        &mut self,
        _ident: &mut String,
        ty: &mut WithMeta<Type>,
        expr: &mut WithMeta<Expr>,
    ) {
        self.visit_type(ty);
        self.visit_expr(expr);
    }
    fn visit_card(
        &mut self,
        _id: &mut CardId,
        item: &mut WithMeta<Expr>,
        next: &mut WithMeta<Expr>,
    ) {
        self.visit_expr(item);
        self.visit_expr(next);
    }
    fn visit_type(&mut self, _ty: &mut WithMeta<Type>) {}
    fn visit_meta(&mut self, _meta: &mut Meta) {}
}

pub trait TypeVisitorMut {
    fn visit_type(&mut self, ty: &mut WithMeta<Type>) {
        self.super_visit_type(ty);
    }
    fn super_visit_type(&mut self, ty: &mut WithMeta<Type>) {
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
    fn visit_brand(&mut self, _brand: &mut Dson, item: &mut WithMeta<Type>) {
        self.visit_type(item);
    }
    fn visit_real(&mut self) {}
    fn visit_integer(&mut self) {}
    fn visit_rational(&mut self) {}
    fn visit_string(&mut self) {}
    fn visit_trait(&mut self, types: &mut [WithMeta<Function>]) {
        for ty in types {
            self.visit_type(&mut ty.value.parameter);
            self.visit_type(&mut ty.value.body);
        }
    }
    fn visit_effectful(&mut self, ty: &mut WithMeta<Type>, effects: &mut WithMeta<EffectExpr>) {
        self.visit_type(ty);
        self.visit_effect_expr(effects);
    }
    fn visit_infer(&mut self) {}
    fn visit_this(&mut self) {}
    fn visit_product(&mut self, types: &mut Vec<WithMeta<Type>>) {
        for ty in types {
            self.visit_type(ty);
        }
    }
    fn visit_sum(&mut self, types: &mut Vec<WithMeta<Type>>) {
        for ty in types {
            self.visit_type(ty);
        }
    }
    fn visit_function(&mut self, parameter: &mut WithMeta<Type>, body: &mut WithMeta<Type>) {
        self.visit_type(parameter);
        self.visit_type(body);
    }
    fn visit_vector(&mut self, ty: &mut WithMeta<Type>) {
        self.visit_type(ty);
    }
    fn visit_map(&mut self, key: &mut WithMeta<Type>, value: &mut WithMeta<Type>) {
        self.visit_type(key);
        self.visit_type(value);
    }
    fn visit_let(
        &mut self,
        _variable: &mut String,
        definition: &mut WithMeta<Type>,
        body: &mut WithMeta<Type>,
    ) {
        self.visit_type(definition);
        self.visit_type(body);
    }
    fn visit_variable(&mut self, _ident: &mut String) {}
    fn visit_attribute(&mut self, _attr: &mut Dson, item: &mut WithMeta<Type>) {
        self.visit_type(item);
    }
    fn visit_expr(&mut self, _item: &mut WithMeta<Expr>) {}
    fn visit_effect_expr(&mut self, item: &mut WithMeta<EffectExpr>) {
        self.super_visit_effect_expr(item);
    }
    fn super_visit_effect_expr(&mut self, item: &mut WithMeta<EffectExpr>) {
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
    fn visit_effect(&mut self, _effect: &mut WithMeta<Effect>) {}
    fn visit_effect_add(&mut self, exprs: &mut Vec<WithMeta<EffectExpr>>) {
        for expr in exprs {
            self.visit_effect_expr(expr);
        }
    }
    fn visit_effect_sub(
        &mut self,
        minuend: &mut WithMeta<EffectExpr>,
        subtrahend: &mut WithMeta<EffectExpr>,
    ) {
        self.visit_effect_expr(minuend);
        self.visit_effect_expr(subtrahend);
    }
    fn visit_effect_apply(
        &mut self,
        function: &mut WithMeta<Type>,
        arguments: &mut Vec<WithMeta<Type>>,
    ) {
        self.visit_type(function);
        for argument in arguments {
            self.visit_type(argument);
        }
    }
    fn visit_all(
        &mut self,
        _variable: &mut String,
        bound: &mut Option<Box<WithMeta<Type>>>,
        body: &mut WithMeta<Type>,
    ) {
        if let Some(bound) = bound {
            self.visit_type(bound);
        }
        self.visit_type(body);
    }
    fn visit_exists(
        &mut self,
        _variable: &mut String,
        bound: &mut Option<Box<WithMeta<Type>>>,
        body: &mut WithMeta<Type>,
    ) {
        if let Some(bound) = bound {
            self.visit_type(bound);
        }
        self.visit_type(body);
    }
}
