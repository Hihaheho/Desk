mod error;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use ast::span::{Span, Spanned};
use error::HirGenError;
use hir::{
    expr::{Expr, Literal},
    meta::{Id, Meta, WithMeta},
    ty::{Effect, Type},
};

#[derive(Default)]
pub struct HirGen {
    next_id: RefCell<Id>,
    next_span: RefCell<Vec<Span>>,
    pub variables: RefCell<HashMap<String, Id>>,
    pub expr_attrs: RefCell<HashMap<Id, Expr>>,
    brands: RefCell<HashSet<String>>,
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
                attr: None,
                id: self.next_id(),
                span: self.pop_span().unwrap(),
            }),
            value,
        }
    }

    fn effect(
        &self,
        ast::ty::Effect { input, output }: ast::ty::Effect,
    ) -> Result<Effect, HirGenError> {
        Ok(Effect {
            input: self.gen_type(input)?,
            output: self.gen_type(output)?,
        })
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
            ast::ty::Type::Effectful { ty, effects } => self.with_meta(Type::Effectful {
                ty: Box::new(self.gen_type(*ty)?),
                effects: effects
                    .into_iter()
                    .map(|effect| self.effect(effect))
                    .collect::<Result<_, _>>()?,
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
            ast::ty::Type::Brand { brand, item } => {
                if self.brands.borrow().contains(&brand) {
                    self.with_meta(Type::Brand {
                        brand,
                        item: Box::new(self.gen_type(*item)?),
                    })
                } else {
                    self.with_meta(Type::Label {
                        label: brand,
                        item: Box::new(self.gen_type(*item)?),
                    })
                }
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
            })),
            ast::expr::Expr::Hole => self.with_meta(Expr::Literal(Literal::Hole)),
            ast::expr::Expr::Let {
                ty: variable,
                definition,
                expression,
            } => self.with_meta(Expr::Let {
                ty: self.gen_type(variable)?,
                definition: Box::new(self.gen(*definition)?),
                expression: Box::new(self.gen(*expression)?),
            }),
            ast::expr::Expr::Perform { input, output } => self.with_meta(Expr::Perform {
                input: Box::new(self.gen(*input)?),
                output: self.gen_type(output)?,
            }),
            ast::expr::Expr::Handle {
                input,
                output,
                handler,
                expr,
            } => self.with_meta(Expr::Handle {
                input: self.gen_type(input)?,
                output: self.gen_type(output)?,
                handler: Box::new(self.gen(*handler)?),
                expr: Box::new(self.gen(*expr)?),
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
                            parameter,
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
            ast::expr::Expr::Attribute { attr, expr } => {
                self.pop_span();
                let mut ret = self.gen(*expr)?;
                if let Some(meta) = &mut ret.meta {
                    let attr = self.gen(*attr)?.value;
                    meta.attr = Some(Box::new(attr.clone()));
                    self.expr_attrs.borrow_mut().insert(meta.id, attr);
                }
                ret
            }
            ast::expr::Expr::Brand { brands, expr } => {
                brands.into_iter().for_each(|brand| {
                    self.brands.borrow_mut().insert(brand);
                });
                self.gen(*expr)?
            }
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
    use hir::{
        meta::{no_meta, Meta},
        ty::Type,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    fn parse(input: &str) -> Spanned<ast::expr::Expr> {
        use chumsky::prelude::end;
        use chumsky::{Parser, Stream};
        parser::expr::parser()
            .then_ignore(end())
            .parse(Stream::from_iter(
                input.len()..input.len() + 1,
                lexer::lexer()
                    .then_ignore(end())
                    .parse(input)
                    .unwrap()
                    .into_iter(),
            ))
            .unwrap()
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
            Type::Product(types) => {
                Type::Product(types.into_iter().map(|ty| remove_meta_ty(ty)).collect())
            }
            Type::Sum(types) => Type::Sum(types.into_iter().map(|ty| remove_meta_ty(ty)).collect()),
            Type::Function { parameter, body } => Type::Function {
                parameter: Box::new(remove_meta_ty(*parameter)),
                body: Box::new(remove_meta_ty(*body)),
            },
            Type::Array(ty) => Type::Array(Box::new(remove_meta_ty(*ty))),
            Type::Set(ty) => Type::Set(Box::new(remove_meta_ty(*ty))),
            Type::Let { definition, body } => Type::Let {
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
        };
        no_meta(value)
    }
    fn remove_meta(expr: WithMeta<Expr>) -> WithMeta<Expr> {
        let value = match expr.value {
            Expr::Literal(_) => expr.value,
            Expr::Let {
                ty,
                definition,
                expression,
            } => Expr::Let {
                ty: remove_meta_ty(ty),
                definition: Box::new(remove_meta(*definition)),
                expression: Box::new(remove_meta(*expression)),
            },
            Expr::Perform { input, output } => Expr::Perform {
                input: Box::new(remove_meta(*input)),
                output: remove_meta_ty(output),
            },
            Expr::Handle {
                input,
                output,
                handler,
                expr,
            } => Expr::Handle {
                input: remove_meta_ty(input),
                output: remove_meta_ty(output),
                handler: Box::new(remove_meta(*handler)),
                expr: Box::new(remove_meta(*expr)),
            },
            Expr::Apply {
                function,
                arguments,
            } => Expr::Apply {
                function: remove_meta_ty(function),
                arguments: arguments
                    .into_iter()
                    .map(|argument| remove_meta(argument))
                    .collect(),
            },
            Expr::Product(exprs) => {
                Expr::Product(exprs.into_iter().map(|expr| remove_meta(expr)).collect())
            }
            Expr::Typed { ty, expr } => Expr::Typed {
                ty: remove_meta_ty(ty),
                expr: Box::new(remove_meta(*expr)),
            },
            Expr::Function { parameter, body } => Expr::Function {
                parameter: remove_meta_ty(parameter),
                body: Box::new(remove_meta(*body)),
            },
            Expr::Array(exprs) => {
                Expr::Array(exprs.into_iter().map(|expr| remove_meta(expr)).collect())
            }
            Expr::Set(exprs) => {
                Expr::Set(exprs.into_iter().map(|expr| remove_meta(expr)).collect())
            }
        };
        no_meta(value)
    }

    #[test]
    fn test() {
        let gen = HirGen::default();
        assert_eq!(
            gen.gen((
                ast::expr::Expr::Apply {
                    function: (ast::ty::Type::Number, 3..10),
                    arguments: vec![(
                        ast::expr::Expr::Attribute {
                            attr: Box::new((
                                ast::expr::Expr::Literal(ast::expr::Literal::Int(3)),
                                24..25
                            )),
                            expr: Box::new((ast::expr::Expr::Hole, 26..27)),
                        },
                        24..27
                    )],
                },
                0..27
            ),),
            Ok(WithMeta {
                meta: Some(Meta {
                    attr: None,
                    id: 3,
                    span: 0..27
                }),
                value: Expr::Apply {
                    function: WithMeta {
                        meta: Some(Meta {
                            attr: None,
                            id: 0,
                            span: 3..10
                        }),
                        value: Type::Number
                    },
                    arguments: vec![WithMeta {
                        meta: Some(Meta {
                            attr: Some(Box::new(Expr::Literal(Literal::Int(3)))),
                            id: 1,
                            span: 26..27
                        }),
                        value: Expr::Literal(Literal::Hole)
                    }],
                },
            })
        );

        assert_eq!(
            gen.expr_attrs.borrow_mut().get(&1),
            Some(&Expr::Literal(Literal::Int(3)))
        );
    }

    #[test]
    fn label_and_brand() {
        let expr = parse(
            r#"
        $ <@brand 'number> ~
        'brand brand ~
        $ <@brand 'number> ~
        <@label 'number>
        "#,
        );

        let gen = HirGen::default();
        assert_eq!(
            remove_meta(gen.gen(expr).unwrap()),
            no_meta(Expr::Let {
                ty: no_meta(Type::Infer),
                definition: Box::new(no_meta(Expr::Apply {
                    function: no_meta(Type::Label {
                        label: "brand".into(),
                        item: Box::new(no_meta(Type::Number)),
                    }),
                    arguments: vec![],
                })),
                expression: Box::new(no_meta(Expr::Let {
                    ty: no_meta(Type::Infer),
                    definition: Box::new(no_meta(Expr::Apply {
                        function: no_meta(Type::Brand {
                            brand: "brand".into(),
                            item: Box::new(no_meta(Type::Number)),
                        }),
                        arguments: vec![],
                    })),
                    expression: Box::new(no_meta(Expr::Apply {
                        function: no_meta(Type::Label {
                            label: "label".into(),
                            item: Box::new(no_meta(Type::Number)),
                        }),
                        arguments: vec![],
                    }))
                }))
            })
        )
    }
}
