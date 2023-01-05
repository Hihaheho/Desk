use dson::Dson;
use ids::{LinkName, NodeId};

use crate::{
    expr::{Expr, Handler, Literal, MapElem, MatchCase},
    meta::{Meta, WithMeta},
    ty::{Effect, EffectExpr, Function, Type},
};

pub trait HirVisitor {
    fn visit_expr(&mut self, expr: &WithMeta<Expr>) {
        self.super_visit_expr(expr)
    }
    fn super_visit_expr(&mut self, expr: &WithMeta<Expr>) {
        match &expr.value {
            Expr::Literal(literal) => self.visit_literal(literal),
            Expr::Do { stmt, expr } => self.visit_do(stmt, expr),
            Expr::Let {
                definition,
                expr: expression,
            } => self.visit_let(definition, expression),
            Expr::Perform { input, output } => self.visit_perform(input, output),
            Expr::Continue { input, output } => self.visit_continue(input, output),
            Expr::Handle { handlers, expr } => self.visit_handle(handlers, expr),
            Expr::Apply {
                function,
                link_name,
                arguments,
            } => self.visit_apply(function, link_name, arguments),
            Expr::Product(exprs) => self.visit_product(exprs),
            Expr::Match { of, cases } => self.visit_match(of, cases),
            Expr::Typed { ty, item } => self.visit_typed(ty, item),
            Expr::Function { parameter, body } => self.visit_function(parameter, body),
            Expr::Vector(exprs) => self.visit_vector(exprs),
            Expr::Map(elems) => self.visit_map(elems),
            Expr::Label { label, item } => self.visit_label(label, item),
            Expr::Brand { brand, item } => self.visit_brand(brand, item),
        }
    }

    fn visit_literal(&mut self, _literal: &Literal) {}
    fn visit_do(&mut self, stmt: &WithMeta<Expr>, expr: &WithMeta<Expr>) {
        self.visit_expr(stmt);
        self.visit_expr(expr);
    }
    fn visit_let(&mut self, definition: &WithMeta<Expr>, expression: &WithMeta<Expr>) {
        self.visit_expr(definition);
        self.visit_expr(expression);
    }
    fn visit_perform(&mut self, input: &WithMeta<Expr>, output: &WithMeta<Type>) {
        self.visit_expr(input);
        self.visit_type(output);
    }
    fn visit_continue(&mut self, input: &WithMeta<Expr>, output: &WithMeta<Type>) {
        self.visit_expr(input);
        self.visit_type(output);
    }
    fn visit_handle(&mut self, handlers: &[WithMeta<Handler>], expr: &WithMeta<Expr>) {
        for handler in handlers {
            self.visit_handler(handler);
        }
        self.visit_expr(expr);
    }
    fn visit_handler(&mut self, handler: &WithMeta<Handler>) {
        self.visit_type(&handler.value.effect.input);
        self.visit_type(&handler.value.effect.output);
        self.visit_expr(&handler.value.handler);
    }
    fn visit_apply(
        &mut self,
        function: &WithMeta<Type>,
        _link_name: &LinkName,
        arguments: &[WithMeta<Expr>],
    ) {
        self.visit_type(function);
        for argument in arguments {
            self.visit_expr(argument);
        }
    }
    fn visit_product(&mut self, exprs: &[WithMeta<Expr>]) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }
    fn visit_match(&mut self, of: &WithMeta<Expr>, cases: &[WithMeta<MatchCase>]) {
        self.visit_expr(of);
        for case in cases {
            self.visit_match_case(case);
        }
    }
    fn visit_match_case(&mut self, case: &WithMeta<MatchCase>) {
        self.visit_type(&case.value.ty);
        self.visit_expr(&case.value.expr);
    }
    fn visit_typed(&mut self, ty: &WithMeta<Type>, item: &WithMeta<Expr>) {
        self.visit_type(ty);
        self.visit_expr(item);
    }
    fn visit_function(&mut self, parameter: &WithMeta<Type>, body: &WithMeta<Expr>) {
        self.visit_type(parameter);
        self.visit_expr(body);
    }
    fn visit_vector(&mut self, exprs: &[WithMeta<Expr>]) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }
    fn visit_map(&mut self, elems: &[WithMeta<MapElem>]) {
        for elem in elems {
            self.visit_map_elem(elem);
        }
    }
    fn visit_map_elem(&mut self, elem: &WithMeta<MapElem>) {
        self.visit_expr(&elem.value.key);
        self.visit_expr(&elem.value.value);
    }
    fn visit_label(&mut self, _label: &Dson, item: &WithMeta<Expr>) {
        self.visit_expr(item);
    }
    fn visit_brand(&mut self, _brand: &Dson, item: &WithMeta<Expr>) {
        self.visit_expr(item);
    }
    fn visit_type(&mut self, _ty: &WithMeta<Type>) {}
}

pub trait HirVisitorMut {
    fn visit_expr(&mut self, expr: &mut WithMeta<Expr>) {
        self.super_visit_expr(expr);
    }
    fn super_visit_expr(&mut self, expr: &mut WithMeta<Expr>) {
        self.visit_meta(&mut expr.meta);
        match &mut expr.value {
            Expr::Literal(literal) => self.visit_literal(literal),
            Expr::Do { stmt, expr } => self.visit_do(stmt, expr),
            Expr::Let {
                definition,
                expr: expression,
            } => self.visit_let(definition, expression),
            Expr::Perform { input, output } => self.visit_perform(input, output),
            Expr::Continue { input, output } => self.visit_continue(input, output),
            Expr::Handle { handlers, expr } => self.visit_handle(handlers, expr),
            Expr::Apply {
                function,
                link_name,
                arguments,
            } => self.visit_apply(function, link_name, arguments),
            Expr::Product(exprs) => self.visit_product(exprs),
            Expr::Match { of, cases } => self.visit_match(of, cases),
            Expr::Typed { ty, item } => self.visit_typed(ty, item),
            Expr::Function { parameter, body } => self.visit_function(parameter, body),
            Expr::Vector(exprs) => self.visit_vector(exprs),
            Expr::Map(elems) => self.visit_map(elems),
            Expr::Label { label, item } => self.visit_label(label, item),
            Expr::Brand { brand, item } => self.visit_brand(brand, item),
        }
    }
    fn visit_literal(&mut self, _literal: &Literal) {}
    fn visit_do(&mut self, stmt: &mut WithMeta<Expr>, expr: &mut WithMeta<Expr>) {
        self.visit_expr(stmt);
        self.visit_expr(expr);
    }
    fn visit_let(&mut self, definition: &mut WithMeta<Expr>, expression: &mut WithMeta<Expr>) {
        self.visit_expr(definition);
        self.visit_expr(expression);
    }
    fn visit_perform(&mut self, input: &mut WithMeta<Expr>, output: &mut WithMeta<Type>) {
        self.visit_expr(input);
        self.visit_type(output);
    }
    fn visit_continue(&mut self, input: &mut WithMeta<Expr>, output: &mut WithMeta<Type>) {
        self.visit_expr(input);
        self.visit_type(output);
    }
    fn visit_handle(&mut self, handlers: &mut [WithMeta<Handler>], expr: &mut WithMeta<Expr>) {
        for handler in handlers {
            self.visit_handler(handler);
        }
        self.visit_expr(expr);
    }
    fn visit_handler(&mut self, handler: &mut WithMeta<Handler>) {
        self.visit_meta(&mut handler.meta);
        self.visit_expr(&mut handler.value.handler);
    }
    fn visit_apply(
        &mut self,
        function: &mut WithMeta<Type>,
        _link_name: &mut LinkName,
        arguments: &mut [WithMeta<Expr>],
    ) {
        self.visit_type(function);
        for argument in arguments {
            self.visit_expr(argument);
        }
    }
    fn visit_product(&mut self, exprs: &mut [WithMeta<Expr>]) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }
    fn visit_match(&mut self, of: &mut WithMeta<Expr>, cases: &mut [WithMeta<MatchCase>]) {
        self.visit_expr(of);
        for case in cases {
            self.visit_match_case(case);
        }
    }
    fn visit_match_case(&mut self, case: &mut WithMeta<MatchCase>) {
        self.visit_meta(&mut case.meta);
        self.visit_expr(&mut case.value.expr);
    }
    fn visit_typed(&mut self, ty: &mut WithMeta<Type>, item: &mut WithMeta<Expr>) {
        self.visit_type(ty);
        self.visit_expr(item);
    }
    fn visit_function(&mut self, parameter: &mut WithMeta<Type>, body: &mut WithMeta<Expr>) {
        self.visit_type(parameter);
        self.visit_expr(body);
    }
    fn visit_vector(&mut self, exprs: &mut [WithMeta<Expr>]) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }
    fn visit_map(&mut self, elems: &mut [WithMeta<MapElem>]) {
        for elem in elems {
            self.visit_map_elem(elem);
        }
    }
    fn visit_map_elem(&mut self, elem: &mut WithMeta<MapElem>) {
        self.visit_meta(&mut elem.meta);
        self.visit_expr(&mut elem.value.key);
        self.visit_expr(&mut elem.value.value);
    }
    fn visit_label(&mut self, _label: &mut Dson, item: &mut WithMeta<Expr>) {
        self.visit_expr(item);
    }
    fn visit_brand(&mut self, _brand: &mut Dson, item: &mut WithMeta<Expr>) {
        self.visit_expr(item);
    }
    fn visit_type(&mut self, _ty: &mut WithMeta<Type>) {
        self.visit_meta(&mut _ty.meta);
    }
    fn visit_meta(&mut self, _meta: &mut Meta) {}
}

pub trait TypeVisitorMut {
    fn visit_type(&mut self, ty: &mut WithMeta<Type>) {
        self.super_visit_type(ty);
    }
    fn super_visit_type(&mut self, ty: &mut WithMeta<Type>) {
        match &mut ty.value {
            Type::Real => self.visit_real(),
            Type::Rational => self.visit_rational(),
            Type::Integer => self.visit_integer(),
            Type::String => self.visit_string(),
            Type::Trait(trait_) => self.visit_trait(trait_),
            Type::Effectful { ty, effects } => self.visit_effectful(ty, effects),
            Type::Infer => self.visit_infer(),
            Type::This => self.visit_this(),
            Type::Product(types) => self.visit_product(types),
            Type::Sum(types) => self.visit_sum(types),
            Type::Function(function) => self.visit_function(function),
            Type::Vector(ty) => self.visit_vector(ty),
            Type::Map { key, value } => self.visit_map(key, value),
            Type::Let {
                variable,
                body,
                definition,
            } => self.visit_let(variable, body, definition),
            Type::Variable(ident) => self.visit_variable(ident),
            Type::Brand { brand, item } => self.visit_brand(brand, item),
            Type::Label { label, item } => self.visit_label(label, item),
            Type::Forall {
                variable,
                bound,
                body,
            } => self.visit_forall(variable, bound, body),
            Type::Exists {
                variable,
                bound,
                body,
            } => self.visit_exists(variable, bound, body),
        }
    }
    fn visit_real(&mut self) {}
    fn visit_rational(&mut self) {}
    fn visit_integer(&mut self) {}
    fn visit_string(&mut self) {}
    fn visit_trait(&mut self, trait_: &mut [WithMeta<Function>]) {
        for function in trait_ {
            self.visit_function(&mut function.value);
        }
    }
    fn visit_function(&mut self, function: &mut Function) {
        self.visit_type(&mut function.parameter);
        self.visit_type(&mut function.body);
    }
    fn visit_effectful(&mut self, ty: &mut WithMeta<Type>, effects: &mut WithMeta<EffectExpr>) {
        self.visit_type(ty);
        self.visit_effect_expr(effects);
    }
    fn visit_infer(&mut self) {}
    fn visit_this(&mut self) {}
    fn visit_product(&mut self, types: &mut [WithMeta<Type>]) {
        for ty in types {
            self.visit_type(ty);
        }
    }
    fn visit_sum(&mut self, types: &mut [WithMeta<Type>]) {
        for ty in types {
            self.visit_type(ty);
        }
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
        body: &mut WithMeta<Type>,
        definition: &mut WithMeta<Type>,
    ) {
        self.visit_type(body);
        self.visit_type(definition);
    }
    fn visit_variable(&mut self, _ident: &mut String) {}
    fn visit_brand(&mut self, _brand: &mut Dson, item: &mut WithMeta<Type>) {
        self.visit_type(item);
    }
    fn visit_label(&mut self, _label: &mut Dson, item: &mut WithMeta<Type>) {
        self.visit_type(item);
    }
    fn visit_forall(
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
    fn visit_effect_expr(&mut self, effect_expr: &mut WithMeta<EffectExpr>) {
        match &mut effect_expr.value {
            EffectExpr::Effects(effects) => {
                for effect in effects {
                    self.visit_effect(effect);
                }
            }
            EffectExpr::Add(exprs) => {
                for expr in exprs {
                    self.visit_effect_expr(expr);
                }
            }
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => {
                self.visit_effect_expr(minuend);
                self.visit_effect_expr(subtrahend);
            }
            EffectExpr::Apply {
                function,
                arguments,
            } => {
                self.visit_type(function);
                for argument in arguments {
                    self.visit_type(argument);
                }
            }
        }
    }
    fn visit_effect(&mut self, effect: &mut WithMeta<Effect>) {
        self.visit_type(&mut effect.value.input);
        self.visit_type(&mut effect.value.output);
    }
}

pub trait TypeVisitor {
    fn visit_type(&mut self, ty: &WithMeta<Type>) {
        self.super_visit_type(ty);
    }
    fn super_visit_type(&mut self, ty: &WithMeta<Type>) {
        match &ty.value {
            Type::Real => self.visit_real(),
            Type::Rational => self.visit_rational(),
            Type::Integer => self.visit_integer(),
            Type::String => self.visit_string(),
            Type::Trait(trait_) => self.visit_trait(trait_),
            Type::Effectful { ty, effects } => self.visit_effectful(ty, effects),
            Type::Infer => self.visit_infer(),
            Type::This => self.visit_this(),
            Type::Product(types) => self.visit_product(types),
            Type::Sum(types) => self.visit_sum(types),
            Type::Function(function) => self.visit_function(function),
            Type::Vector(ty) => self.visit_vector(ty),
            Type::Map { key, value } => self.visit_map(key, value),
            Type::Let {
                variable,
                body,
                definition,
            } => self.visit_let(variable, body, definition),
            Type::Variable(ident) => self.visit_variable(ident),
            Type::Brand { brand, item } => self.visit_brand(brand, item),
            Type::Label { label, item } => self.visit_label(label, item),
            Type::Forall {
                variable,
                bound,
                body,
            } => self.visit_forall(variable, bound, body),
            Type::Exists {
                variable,
                bound,
                body,
            } => self.visit_exists(variable, bound, body),
        }
    }
    fn visit_real(&mut self) {}
    fn visit_rational(&mut self) {}
    fn visit_integer(&mut self) {}
    fn visit_string(&mut self) {}
    fn visit_trait(&mut self, trait_: &[WithMeta<Function>]) {
        for function in trait_ {
            self.visit_function(&function.value);
        }
    }
    fn visit_function(&mut self, function: &Function) {
        self.visit_type(&function.parameter);
        self.visit_type(&function.body);
    }
    fn visit_effectful(&mut self, ty: &WithMeta<Type>, effects: &WithMeta<EffectExpr>) {
        self.visit_type(ty);
        self.visit_effect_expr(effects);
    }
    fn visit_infer(&mut self) {}
    fn visit_this(&mut self) {}
    fn visit_product(&mut self, types: &[WithMeta<Type>]) {
        for ty in types {
            self.visit_type(ty);
        }
    }
    fn visit_sum(&mut self, types: &[WithMeta<Type>]) {
        for ty in types {
            self.visit_type(ty);
        }
    }
    fn visit_vector(&mut self, ty: &WithMeta<Type>) {
        self.visit_type(ty);
    }
    fn visit_map(&mut self, key: &WithMeta<Type>, value: &WithMeta<Type>) {
        self.visit_type(key);
        self.visit_type(value);
    }
    fn visit_let(&mut self, _variable: &str, body: &WithMeta<Type>, definition: &WithMeta<Type>) {
        self.visit_type(body);
        self.visit_type(definition);
    }
    fn visit_variable(&mut self, _ident: &str) {}
    fn visit_brand(&mut self, _brand: &Dson, item: &WithMeta<Type>) {
        self.visit_type(item);
    }
    fn visit_label(&mut self, _label: &Dson, item: &WithMeta<Type>) {
        self.visit_type(item);
    }
    fn visit_forall(
        &mut self,
        _variable: &str,
        bound: &Option<Box<WithMeta<Type>>>,
        body: &WithMeta<Type>,
    ) {
        if let Some(bound) = bound {
            self.visit_type(bound);
        }
        self.visit_type(body);
    }
    fn visit_exists(
        &mut self,
        _variable: &str,
        bound: &Option<Box<WithMeta<Type>>>,
        body: &WithMeta<Type>,
    ) {
        if let Some(bound) = bound {
            self.visit_type(bound);
        }
        self.visit_type(body);
    }
    fn visit_effect_expr(&mut self, effect_expr: &WithMeta<EffectExpr>) {
        match &effect_expr.value {
            EffectExpr::Effects(effects) => {
                for effect in effects {
                    self.visit_effect(effect);
                }
            }
            EffectExpr::Add(exprs) => {
                for expr in exprs {
                    self.visit_effect_expr(expr);
                }
            }
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => {
                self.visit_effect_expr(minuend);
                self.visit_effect_expr(subtrahend);
            }
            EffectExpr::Apply {
                function,
                arguments,
            } => {
                self.visit_type(function);
                for argument in arguments {
                    self.visit_type(argument);
                }
            }
        }
    }
    fn visit_effect(&mut self, effect: &WithMeta<Effect>) {
        self.visit_type(&effect.value.input);
        self.visit_type(&effect.value.output);
    }
}

fn remove_meta_ty(ty: &mut WithMeta<Type>) {
    struct RemoveMetaTy;
    impl TypeVisitorMut for RemoveMetaTy {
        fn visit_type(&mut self, ty: &mut WithMeta<Type>) {
            ty.meta.id = NodeId::default();
            self.super_visit_type(ty);
        }
    }
    RemoveMetaTy.visit_type(ty);
}
pub fn remove_meta(mut expr: WithMeta<Expr>) -> WithMeta<Expr> {
    struct RemoveMeta;
    impl HirVisitorMut for RemoveMeta {
        fn visit_meta(&mut self, meta: &mut Meta) {
            meta.id = NodeId::default();
        }
        fn visit_type(&mut self, ty: &mut WithMeta<Type>) {
            remove_meta_ty(ty);
        }
    }
    RemoveMeta.visit_expr(&mut expr);
    expr
}
