use dson::Dson;
use ids::LinkName;

use crate::{
    expr::{Expr, Handler, Literal, MapElem, MatchCase},
    meta::{dummy_meta, WithMeta},
    ty::{Function, Type},
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
                expression,
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
    fn visit_perform(&mut self, input: &WithMeta<Expr>, output: &Option<WithMeta<Type>>) {
        self.visit_expr(input);
        if let Some(output) = output {
            self.visit_type(output);
        }
    }
    fn visit_continue(&mut self, input: &WithMeta<Expr>, output: &Option<WithMeta<Type>>) {
        self.visit_expr(input);
        if let Some(output) = output {
            self.visit_type(output);
        }
    }
    fn visit_handle(&mut self, handlers: &[Handler], expr: &WithMeta<Expr>) {
        for handler in handlers {
            self.visit_handler(handler);
        }
        self.visit_expr(expr);
    }
    fn visit_handler(&mut self, handler: &Handler) {
        self.visit_expr(&handler.handler);
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
    fn visit_match(&mut self, of: &WithMeta<Expr>, cases: &[MatchCase]) {
        self.visit_expr(of);
        for case in cases {
            self.visit_match_case(case);
        }
    }
    fn visit_match_case(&mut self, case: &MatchCase) {
        self.visit_expr(&case.expr);
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
    fn visit_map(&mut self, elems: &[MapElem]) {
        for elem in elems {
            self.visit_map_elem(elem);
        }
    }
    fn visit_map_elem(&mut self, elem: &MapElem) {
        self.visit_expr(&elem.key);
        self.visit_expr(&elem.value);
    }
    fn visit_label(&mut self, _label: &Dson, item: &WithMeta<Expr>) {
        self.visit_expr(item);
    }
    fn visit_brand(&mut self, _brand: &Dson, item: &WithMeta<Expr>) {
        self.visit_expr(item);
    }
    fn visit_type(&mut self, _ty: &WithMeta<Type>) {}
}

fn remove_meta_ty(ty: WithMeta<Type>) -> WithMeta<Type> {
    let value = match ty.value {
        Type::Number => ty.value,
        Type::String => ty.value,
        Type::Trait(_) => todo!(),
        Type::Effectful { ty, effects } => Type::Effectful {
            ty: Box::new(remove_meta_ty(*ty)),
            effects,
        },
        Type::Infer => ty.value,
        Type::This => ty.value,
        Type::Product(types) => Type::Product(types.into_iter().map(remove_meta_ty).collect()),
        Type::Sum(types) => Type::Sum(types.into_iter().map(remove_meta_ty).collect()),
        Type::Function(function) => Type::Function(Box::new(Function {
            parameter: remove_meta_ty(function.parameter),
            body: remove_meta_ty(function.body),
        })),
        Type::Vector(ty) => Type::Vector(Box::new(remove_meta_ty(*ty))),
        Type::Map { key, value } => Type::Map {
            key: Box::new(remove_meta_ty(*key)),
            value: Box::new(remove_meta_ty(*value)),
        },
        Type::Let {
            variable,
            definition,
            body,
        } => Type::Let {
            variable,
            definition: Box::new(remove_meta_ty(*definition)),
            body: Box::new(remove_meta_ty(*body)),
        },
        Type::Variable(_) => ty.value,
        Type::BoundedVariable { bound, identifier } => Type::BoundedVariable {
            bound: Box::new(remove_meta_ty(*bound)),
            identifier,
        },
        Type::Brand { brand, item } => Type::Brand {
            brand,
            item: Box::new(remove_meta_ty(*item)),
        },
        Type::Label { label, item } => Type::Label {
            label,
            item: Box::new(remove_meta_ty(*item)),
        },
        Type::Forall {
            variable,
            bound,
            body,
        } => Type::Forall {
            variable,
            bound: bound.map(|bound| Box::new(remove_meta_ty(*bound))),
            body: Box::new(remove_meta_ty(*body)),
        },
        Type::Exists {
            variable,
            bound,
            body,
        } => Type::Exists {
            variable,
            bound: bound.map(|bound| Box::new(remove_meta_ty(*bound))),
            body: Box::new(remove_meta_ty(*body)),
        },
    };
    dummy_meta(value)
}
pub fn remove_meta(expr: WithMeta<Expr>) -> WithMeta<Expr> {
    let value = match expr.value {
        Expr::Literal(_) => expr.value,
        Expr::Do { stmt, expr } => Expr::Do {
            stmt: Box::new(remove_meta(*stmt)),
            expr: Box::new(remove_meta(*expr)),
        },
        Expr::Let {
            definition,
            expression,
        } => Expr::Let {
            definition: Box::new(remove_meta(*definition)),
            expression: Box::new(remove_meta(*expression)),
        },
        Expr::Perform { input, output } => Expr::Perform {
            input: Box::new(remove_meta(*input)),
            output: output.map(remove_meta_ty),
        },
        Expr::Continue { input, output } => Expr::Continue {
            input: Box::new(remove_meta(*input)),
            output: output.map(remove_meta_ty),
        },
        Expr::Handle { handlers, expr } => Expr::Handle {
            handlers: handlers
                .into_iter()
                .map(
                    |Handler {
                         input,
                         output,
                         handler,
                     }| Handler {
                        input: remove_meta_ty(input),
                        output: remove_meta_ty(output),
                        handler: remove_meta(handler),
                    },
                )
                .collect(),
            expr: Box::new(remove_meta(*expr)),
        },
        Expr::Apply {
            function,
            link_name,
            arguments,
        } => Expr::Apply {
            function: remove_meta_ty(function),
            link_name,
            arguments: arguments.into_iter().map(remove_meta).collect(),
        },
        Expr::Product(exprs) => Expr::Product(exprs.into_iter().map(remove_meta).collect()),
        Expr::Typed { ty, item: expr } => Expr::Typed {
            ty: remove_meta_ty(ty),
            item: Box::new(remove_meta(*expr)),
        },
        Expr::Function { parameter, body } => Expr::Function {
            parameter: remove_meta_ty(parameter),
            body: Box::new(remove_meta(*body)),
        },
        Expr::Vector(exprs) => Expr::Vector(exprs.into_iter().map(remove_meta).collect()),
        Expr::Map(elems) => Expr::Map(
            elems
                .into_iter()
                .map(|elem| MapElem {
                    key: remove_meta(elem.key),
                    value: remove_meta(elem.value),
                })
                .collect(),
        ),
        Expr::Match { of, cases } => Expr::Match {
            of: Box::new(remove_meta(*of)),
            cases: cases
                .into_iter()
                .map(|MatchCase { ty, expr }| MatchCase {
                    ty: remove_meta_ty(ty),
                    expr: remove_meta(expr),
                })
                .collect(),
        },
        Expr::Label { label, item: body } => Expr::Label {
            label,
            item: Box::new(remove_meta(*body)),
        },
        Expr::Brand { brand, item: body } => Expr::Brand {
            brand,
            item: Box::new(remove_meta(*body)),
        },
    };
    let mut with_meta = dummy_meta(value);
    with_meta.meta.attrs = expr.meta.attrs;
    with_meta
}
