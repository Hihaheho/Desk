use dson::Dson;

use crate::{MapElem, TypedHir};

pub trait TypedHirVisitor {
    fn visit(&mut self, hir: &TypedHir) {
        match &hir.expr {
            crate::Expr::Literal(literal) => self.visit_literal(literal),
            crate::Expr::Do { stmt, expr } => {
                self.visit_do(stmt, expr);
            }
            crate::Expr::Match { input, cases } => self.visit_match(input, cases),
            crate::Expr::Let { definition, body } => self.visit_let(definition, body),
            crate::Expr::Perform(perform) => self.visit_perform(perform),
            crate::Expr::Handle { handlers, expr } => self.visit_handle(handlers, expr),
            crate::Expr::Apply {
                function,
                link_name,
                arguments,
            } => self.visit_apply(function, link_name, arguments),
            crate::Expr::Product(exprs) => self.visit_product(exprs),
            crate::Expr::Function { parameter, body } => self.visit_function(parameter, body),
            crate::Expr::Vector(exprs) => self.visit_vector(exprs),
            crate::Expr::Map(elems) => self.visit_map(elems),
            crate::Expr::Label { label, item } => self.visit_label(label, item),
        }
    }

    fn visit_literal(&mut self, _literal: &crate::Literal) {}
    fn visit_do(&mut self, stmt: &TypedHir, expr: &TypedHir) {
        self.visit(stmt);
        self.visit(expr);
    }
    fn visit_match(&mut self, input: &crate::TypedHir, cases: &[crate::MatchCase]) {
        self.visit(input);
        for case in cases {
            self.visit(&case.expr);
        }
    }
    fn visit_let(&mut self, definition: &crate::TypedHir, body: &crate::TypedHir) {
        self.visit(definition);
        self.visit(body);
    }
    fn visit_perform(&mut self, perform: &crate::TypedHir) {
        self.visit(perform);
    }
    fn visit_handle(&mut self, handlers: &[crate::Handler], expr: &crate::TypedHir) {
        for handler in handlers {
            self.visit_handler(&handler.effect, &handler.handler);
        }
        self.visit(expr);
    }
    fn visit_handler(&mut self, effect: &crate::Effect, handler: &crate::TypedHir) {
        self.visit_effect(&effect.input, &effect.output);
        self.visit(handler);
    }
    fn visit_effect(&mut self, input: &crate::Type, output: &crate::Type) {
        self.visit_type(input);
        self.visit_type(output);
    }
    fn visit_type(&mut self, _ty: &crate::Type) {}
    fn visit_apply(
        &mut self,
        function: &crate::Type,
        link_name: &crate::LinkName,
        arguments: &[crate::TypedHir],
    ) {
        self.visit_type(function);
        self.visit_link_name(link_name);
        for argument in arguments {
            self.visit(argument);
        }
    }
    fn visit_link_name(&mut self, _link_name: &crate::LinkName) {}
    fn visit_product(&mut self, exprs: &[crate::TypedHir]) {
        for expr in exprs {
            self.visit(expr);
        }
    }
    fn visit_function(&mut self, parameter: &crate::Type, body: &crate::TypedHir) {
        self.visit_type(parameter);
        self.visit(body);
    }
    fn visit_vector(&mut self, exprs: &[crate::TypedHir]) {
        for expr in exprs {
            self.visit(expr);
        }
    }
    fn visit_map(&mut self, elems: &[MapElem]) {
        for elem in elems {
            self.visit(&elem.key);
            self.visit(&elem.value);
        }
    }
    fn visit_label(&mut self, _label: &Dson, item: &crate::TypedHir) {
        self.visit(item);
    }
}

pub trait TypedHirVisitorMut {
    fn visit(&mut self, hir: &mut TypedHir) {
        self.super_visit(hir);
    }
    fn super_visit(&mut self, hir: &mut TypedHir) {
        match &mut hir.expr {
            crate::Expr::Literal(literal) => self.visit_literal(literal),
            crate::Expr::Do { stmt, expr } => {
                self.visit_do(stmt, expr);
            }
            crate::Expr::Match { input, cases } => self.visit_match(input, cases),
            crate::Expr::Let { definition, body } => self.visit_let(definition, body),
            crate::Expr::Perform(perform) => self.visit_perform(perform),
            crate::Expr::Handle { handlers, expr } => self.visit_handle(handlers, expr),
            crate::Expr::Apply {
                function,
                link_name,
                arguments,
            } => self.visit_apply(function, link_name, arguments),
            crate::Expr::Product(exprs) => self.visit_product(exprs),
            crate::Expr::Function { parameter, body } => self.visit_function(parameter, body),
            crate::Expr::Vector(exprs) => self.visit_vector(exprs),
            crate::Expr::Map(elems) => self.visit_map(elems),
            crate::Expr::Label { label, item } => self.visit_label(label, item),
        }
    }

    fn visit_literal(&mut self, _literal: &mut crate::Literal) {}
    fn visit_do(&mut self, stmt: &mut TypedHir, expr: &mut TypedHir) {
        self.visit(stmt);
        self.visit(expr);
    }
    fn visit_match(&mut self, input: &mut crate::TypedHir, cases: &mut [crate::MatchCase]) {
        self.visit(input);
        for case in cases {
            self.visit(&mut case.expr);
        }
    }
    fn visit_let(&mut self, definition: &mut crate::TypedHir, body: &mut crate::TypedHir) {
        self.visit(definition);
        self.visit(body);
    }
    fn visit_perform(&mut self, perform: &mut crate::TypedHir) {
        self.visit(perform);
    }
    fn visit_handle(&mut self, handlers: &mut [crate::Handler], expr: &mut crate::TypedHir) {
        for handler in handlers {
            self.visit_handler(&mut handler.effect, &mut handler.handler);
        }
        self.visit(expr);
    }
    fn visit_handler(&mut self, effect: &mut crate::Effect, handler: &mut crate::TypedHir) {
        self.visit_effect(&mut effect.input, &mut effect.output);
        self.visit(handler);
    }
    fn visit_effect(&mut self, input: &mut crate::Type, output: &mut crate::Type) {
        self.visit_type(input);
        self.visit_type(output);
    }
    fn visit_type(&mut self, _ty: &mut crate::Type) {}
    fn visit_apply(
        &mut self,
        function: &mut crate::Type,
        link_name: &mut crate::LinkName,
        arguments: &mut [crate::TypedHir],
    ) {
        self.visit_type(function);
        self.visit_link_name(link_name);
        for argument in arguments {
            self.visit(argument);
        }
    }
    fn visit_link_name(&mut self, _link_name: &mut crate::LinkName) {}
    fn visit_product(&mut self, exprs: &mut [crate::TypedHir]) {
        for expr in exprs {
            self.visit(expr);
        }
    }
    fn visit_function(&mut self, parameter: &mut crate::Type, body: &mut crate::TypedHir) {
        self.visit_type(parameter);
        self.visit(body);
    }
    fn visit_vector(&mut self, exprs: &mut [crate::TypedHir]) {
        for expr in exprs {
            self.visit(expr);
        }
    }
    fn visit_map(&mut self, elems: &mut [MapElem]) {
        for elem in elems {
            self.visit(&mut elem.key);
            self.visit(&mut elem.value);
        }
    }
    fn visit_label(&mut self, _label: &mut Dson, item: &mut crate::TypedHir) {
        self.visit(item);
    }
}
