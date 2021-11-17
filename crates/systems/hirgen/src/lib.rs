mod error;

use std::{cell::RefCell, collections::HashMap};

use ast::span::{Span, Spanned};
use error::HirGenError;
use hir::{
    expr::{Expr, Literal},
    meta::{Id, Meta, WithMeta},
    ty::{Handler, Type},
};

#[derive(Default)]
pub struct HirGen {
    next_id: RefCell<Id>,
    variables: RefCell<HashMap<String, Id>>,
    next_span: RefCell<Vec<Span>>,
}

impl HirGen {
    pub fn get_id_of(&self, ident: String) -> usize {
        *self
            .variables
            .borrow_mut()
            .entry(ident)
            .or_insert_with(|| self.next_id())
    }

    pub fn next_id(&self) -> Id {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() += 1;
        id
    }

    pub fn with_meta<T: std::fmt::Debug>(&self, value: T) -> WithMeta<T> {
        WithMeta {
            meta: Some(Meta {
                id: self.next_id(),
                span: self.pop_span().unwrap(),
            }),
            value,
        }
    }

    fn handler_type(
        &self,
        ast::ty::Handler { input, output }: ast::ty::Handler,
    ) -> Result<Handler, HirGenError> {
        Ok(Handler {
            input: self.gen_type(input)?,
            output: self.gen_type(output)?,
        })
    }

    pub fn gen_class(&self, ty: Spanned<ast::ty::Type>) -> Result<Vec<Handler>, HirGenError> {
        if let ast::ty::Type::Class(class) = ty.0 {
            Ok(class
                .into_iter()
                .map(|handler| self.handler_type(handler))
                .collect::<Result<_, _>>()?)
        } else {
            Err(HirGenError::ClassExpected { span: ty.1 })
        }
    }

    pub fn gen_type(&self, ty: Spanned<ast::ty::Type>) -> Result<WithMeta<Type>, HirGenError> {
        let (ty, span) = ty;
        self.push_span(span);

        let with_meta = match ty {
            ast::ty::Type::Number => self.with_meta(Type::Number),
            ast::ty::Type::String => self.with_meta(Type::String),
            ast::ty::Type::Trait(types) => self.with_meta(Type::Trait(
                types
                    .into_iter()
                    .map(|ty| self.gen_type(ty))
                    .collect::<Result<_, _>>()?,
            )),
            ast::ty::Type::Class(_handlers) => Err(HirGenError::UnexpectedClass {
                span: self.pop_span().unwrap(),
            })?,
            ast::ty::Type::Effectful {
                class,
                ty,
                handlers,
            } => self.with_meta(Type::Effectful {
                class: self.gen_class(*class)?,
                ty: Box::new(self.gen_type(*ty)?),
                handlers: handlers
                    .into_iter()
                    .map(|handler| self.handler_type(handler))
                    .collect::<Result<_, _>>()?,
            }),
            ast::ty::Type::Effect { class, handler } => self.with_meta(Type::Effect {
                class: self.gen_class(*class)?,
                handler: Box::new(self.handler_type(*handler)?),
            }),
            ast::ty::Type::Infer => self.with_meta(Type::Infer),
            ast::ty::Type::This => self.with_meta(Type::This),
            ast::ty::Type::Alias(alias) => todo!(),
            ast::ty::Type::Product(types) => self.with_meta(Type::Product(
                types
                    .into_iter()
                    .map(|ty| self.gen_type(ty))
                    .collect::<Result<_, _>>()?,
            )),
            ast::ty::Type::Sum(types) => self.with_meta(Type::Sum(
                types
                    .into_iter()
                    .map(|ty| self.gen_type(ty))
                    .collect::<Result<_, _>>()?,
            )),
            ast::ty::Type::Function { parameters, body } => {
                let span = self.pop_span().unwrap();
                parameters
                    .into_iter()
                    .map(|parameter| self.gen_type(parameter))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .try_rfold(self.gen_type(*body)?, |body, parameter| {
                        self.push_span(span.clone());
                        Ok(self.with_meta(Type::Function {
                            parameter: Box::new(parameter),
                            body: Box::new(body),
                        }))
                    })?
            }
            ast::ty::Type::Array(ty) => self.with_meta(Type::Array(Box::new(self.gen_type(*ty)?))),
            ast::ty::Type::Set(ty) => self.with_meta(Type::Set(Box::new(self.gen_type(*ty)?))),
            ast::ty::Type::Let { definition, body } => self.with_meta(Type::Let {
                definition: Box::new(self.gen_type(*definition)?),
                body: Box::new(self.gen_type(*body)?),
            }),
            ast::ty::Type::Variable(ident) => self.with_meta(Type::Variable(self.get_id_of(ident))),
            ast::ty::Type::BoundedVariable { bound, identifier } => {
                self.with_meta(Type::BoundedVariable {
                    bound: Box::new(self.gen_type(*bound)?),
                    identifier,
                })
            }
        };
        Ok(with_meta)
    }

    pub fn gen(&self, ast: Spanned<ast::expr::Expr>) -> Result<WithMeta<Expr>, HirGenError> {
        let (expr, span) = ast;
        self.push_span(span);

        let with_meta = match expr {
            ast::expr::Expr::Literal(literal) => self.with_meta(Expr::Literal(match literal {
                ast::expr::Literal::String(value) => Literal::String(value),
                ast::expr::Literal::Int(value) => Literal::Int(value),
                ast::expr::Literal::Rational(a, b) => Literal::Rational(a, b),
                ast::expr::Literal::Float(value) => Literal::Float(value),
                ast::expr::Literal::Uuid(value) => Literal::Uuid(value),
            })),
            ast::expr::Expr::Let {
                ty: variable,
                definition,
                expression,
            } => self.with_meta(Expr::Let {
                ty: self.gen_type(variable)?,
                definition: Box::new(self.gen(*definition)?),
                expression: Box::new(self.gen(*expression)?),
            }),
            ast::expr::Expr::Perform { effect } => self.with_meta(Expr::Perform {
                effect: Box::new(self.gen(*effect)?),
            }),
            ast::expr::Expr::Effectful {
                class,
                expr,
                handlers,
            } => self.with_meta(Expr::Effectful {
                class: self.gen_type(class)?,
                expr: Box::new(self.gen(*expr)?),
                handlers: handlers
                    .into_iter()
                    .map(|ast::expr::Handler { ty, expr }| {
                        Ok(hir::expr::Handler {
                            ty: self.gen_type(ty)?,
                            expr: self.gen(expr)?,
                        })
                    })
                    .collect::<Result<Vec<hir::expr::Handler>, _>>()?,
            }),
            ast::expr::Expr::Apply {
                function,
                arguments,
            } => self.with_meta(Expr::Apply {
                function: self.gen_type(function)?,
                arguments: arguments
                    .into_iter()
                    .map(|argument| self.gen(argument))
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            ast::expr::Expr::Product(items) => self.with_meta(Expr::Product(
                items
                    .into_iter()
                    .map(|item| self.gen(item))
                    .collect::<Result<_, _>>()?,
            )),
            ast::expr::Expr::Typed { ty, expr } => self.with_meta(Expr::Typed {
                ty: self.gen_type(ty)?,
                expr: Box::new(self.gen(*expr)?),
            }),
            ast::expr::Expr::Hole => self.with_meta(Expr::Hole),
            ast::expr::Expr::Function { parameters, body } => {
                let span = self.pop_span().unwrap();
                parameters
                    .into_iter()
                    .map(|parameter| self.gen_type(parameter))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .try_rfold(self.gen(*body)?, |body, parameter| {
                        self.push_span(span.clone());
                        Ok(self.with_meta(Expr::Function {
                            parameter: Box::new(parameter),
                            body: Box::new(body),
                        }))
                    })?
            }
            ast::expr::Expr::Array(items) => self.with_meta(Expr::Array(
                items
                    .into_iter()
                    .map(|item| self.gen(item))
                    .collect::<Result<_, _>>()?,
            )),
            ast::expr::Expr::Set(items) => self.with_meta(Expr::Set(
                items
                    .into_iter()
                    .map(|item| self.gen(item))
                    .collect::<Result<_, _>>()?,
            )),
            ast::expr::Expr::Module(_) => todo!(),
            ast::expr::Expr::Import { ty, uuid } => todo!(),
            ast::expr::Expr::Export { ty } => todo!(),
        };
        Ok(with_meta)
    }

    pub(crate) fn push_span(&self, span: Span) {
        self.next_span.borrow_mut().push(span);
    }

    pub(crate) fn pop_span(&self) -> Option<Span> {
        self.next_span.borrow_mut().pop()
    }
}

#[cfg(test)]
mod tests {
    use hir::{meta::Meta, ty::Type};

    use super::*;

    #[test]
    fn test() {
        let mut gen = HirGen::default();
        assert_eq!(
            gen.gen((
                ast::expr::Expr::Apply {
                    function: (ast::ty::Type::Number, 3..10),
                    arguments: vec![(ast::expr::Expr::Hole, 26..27)],
                },
                0..27
            ),),
            Ok(WithMeta {
                meta: Some(Meta { id: 2, span: 0..27 }),
                value: Expr::Apply {
                    function: WithMeta {
                        meta: Some(Meta { id: 0, span: 3..10 }),
                        value: Type::Number
                    },
                    arguments: vec![WithMeta {
                        meta: Some(Meta {
                            id: 1,
                            span: 26..27
                        }),
                        value: Expr::Hole
                    }],
                },
            })
        );
    }

    // TODO flatten product and sum
}
