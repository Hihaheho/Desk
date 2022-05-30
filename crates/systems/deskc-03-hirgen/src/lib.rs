mod error;
mod gen_effect_expr;
use ids::{CardId, FileId, NodeId};

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use ast::span::{Span, WithSpan};
use error::HirGenError;
use hir::{
    expr::{Expr, Handler, Literal, MatchCase},
    meta::{Meta, WithMeta},
    ty::Type,
    Hir,
};

pub fn gen_hir(src: &WithSpan<ast::expr::Expr>) -> Result<(HirGen, Hir), HirGenError> {
    let mut hirgen = HirGen::default();
    hirgen.gen_hir(src)?;
    let hir = Hir {
        entrypoint: hirgen.entrypoint.take(),
        cards: hirgen.cards.drain(..).collect(),
    };
    Ok((hirgen, hir))
}

#[derive(Default, Debug)]
pub struct HirGen {
    file_id: FileId,
    next_id: RefCell<usize>,
    next_span: RefCell<Vec<(NodeId, Span)>>,
    pub attrs: RefCell<HashMap<NodeId, Vec<Expr>>>,
    pub type_aliases: RefCell<HashMap<String, Type>>,
    brands: RefCell<HashSet<String>>,
    cards: Vec<(CardId, WithMeta<Expr>)>,
    entrypoint: Option<WithMeta<Expr>>,
}

impl HirGen {
    pub fn gen_type(&self, ty: &WithSpan<ast::ty::Type>) -> Result<WithMeta<Type>, HirGenError> {
        let WithSpan {
            id,
            value: ty,
            span,
        } = ty;
        self.push_span(id.clone(), span.clone());

        let with_meta = match ty {
            ast::ty::Type::Number => self.with_meta(Type::Number),
            ast::ty::Type::String => self.with_meta(Type::String),
            ast::ty::Type::Trait(types) => self.with_meta(Type::Trait(
                types
                    .iter()
                    .map(|ty| self.gen_type(ty))
                    .collect::<Result<_, _>>()?,
            )),
            ast::ty::Type::Effectful { ty, effects } => self.with_meta(Type::Effectful {
                ty: Box::new(self.gen_type(ty)?),
                effects: self.gen_effect_expr(effects)?,
            }),
            ast::ty::Type::Infer => self.with_meta(Type::Infer),
            ast::ty::Type::This => self.with_meta(Type::This),
            ast::ty::Type::Variable(ident) => self.with_meta(
                self.type_aliases
                    .borrow()
                    .get(ident)
                    .cloned()
                    .unwrap_or_else(|| Type::Variable(ident.clone())),
            ),
            ast::ty::Type::Product(types) => self.with_meta(Type::Product(
                types
                    .iter()
                    .map(|ty| self.gen_type(ty))
                    .collect::<Result<_, _>>()?,
            )),
            ast::ty::Type::Sum(types) => self.with_meta(Type::Sum(
                types
                    .iter()
                    .map(|ty| self.gen_type(ty))
                    .collect::<Result<_, _>>()?,
            )),
            ast::ty::Type::Function { parameters, body } => self.with_meta(Type::Function {
                parameters: parameters
                    .iter()
                    .map(|ty| self.gen_type(ty))
                    .collect::<Result<_, _>>()?,
                body: Box::new(self.gen_type(body)?),
            }),
            ast::ty::Type::Vector(ty) => self.with_meta(Type::Vector(Box::new(self.gen_type(ty)?))),
            ast::ty::Type::Set(ty) => self.with_meta(Type::Set(Box::new(self.gen_type(ty)?))),
            ast::ty::Type::Let { variable, body } => self.with_meta(Type::Let {
                variable: variable.clone(),
                body: Box::new(self.gen_type(body)?),
            }),
            ast::ty::Type::BoundedVariable { bound, identifier } => {
                self.with_meta(Type::BoundedVariable {
                    bound: Box::new(self.gen_type(bound)?),
                    identifier: identifier.clone(),
                })
            }
            ast::ty::Type::Brand { brand, item } => {
                if self.brands.borrow().contains(brand) {
                    self.with_meta(Type::Brand {
                        brand: brand.clone(),
                        item: Box::new(self.gen_type(item)?),
                    })
                } else {
                    self.with_meta(Type::Label {
                        label: brand.clone(),
                        item: Box::new(self.gen_type(item)?),
                    })
                }
            }
            ast::ty::Type::Attribute { attr, ty } => {
                self.pop_span();
                let mut ret = self.gen_type(ty)?;
                let attr = self.gen(attr)?.value;
                ret.meta.attrs.push(attr);
                self.attrs
                    .borrow_mut()
                    .insert(ret.id.clone(), ret.meta.attrs.clone());
                ret
            }
            ast::ty::Type::Comment { item, .. } => self.gen_type(item)?,
        };
        Ok(with_meta)
    }

    pub fn gen(&self, ast: &WithSpan<ast::expr::Expr>) -> Result<WithMeta<Expr>, HirGenError> {
        let WithSpan {
            value: expr,
            id,
            span,
        } = ast;
        self.push_span(id.clone(), span.clone());

        let with_meta = match expr {
            ast::expr::Expr::Literal(literal) => self.with_meta(Expr::Literal(match literal {
                ast::expr::Literal::String(value) => Literal::String(value.clone()),
                ast::expr::Literal::Integer(value) => Literal::Integer(*value),
                ast::expr::Literal::Rational(a, b) => Literal::Rational(*a, *b),
                ast::expr::Literal::Float(value) => Literal::Float(*value),
            })),
            ast::expr::Expr::Hole => self.with_meta(Expr::Literal(Literal::Hole)),
            ast::expr::Expr::Let {
                ty: variable,
                definition,
                body: expression,
            } => self.with_meta(Expr::Let {
                ty: self.gen_type(variable)?,
                definition: Box::new(self.gen(definition)?),
                expression: Box::new(self.gen(expression)?),
            }),
            ast::expr::Expr::Perform { input, output } => self.with_meta(Expr::Perform {
                input: Box::new(self.gen(input)?),
                output: self.gen_type(output)?,
            }),
            ast::expr::Expr::Continue { input, output } => self.with_meta(Expr::Continue {
                input: Box::new(self.gen(input)?),
                output: output
                    .as_ref()
                    .map(|output| self.gen_type(output))
                    .transpose()?,
            }),
            ast::expr::Expr::Handle { handlers, expr } => self.with_meta(Expr::Handle {
                handlers: handlers
                    .iter()
                    .map(
                        |ast::expr::Handler {
                             input,
                             output,
                             handler,
                         }| {
                            Ok(Handler {
                                input: self.gen_type(input)?,
                                output: self.gen_type(output)?,
                                handler: self.gen(handler)?,
                            })
                        },
                    )
                    .collect::<Result<Vec<_>, _>>()?,
                expr: Box::new(self.gen(expr)?),
            }),
            ast::expr::Expr::Apply {
                function,
                link_name,
                arguments,
            } => self.with_meta(Expr::Apply {
                function: self.gen_type(function)?,
                link_name: link_name.clone(),
                arguments: arguments
                    .iter()
                    .map(|argument| self.gen(argument))
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            ast::expr::Expr::Product(items) => self.with_meta(Expr::Product(
                items
                    .iter()
                    .map(|item| self.gen(item))
                    .collect::<Result<_, _>>()?,
            )),
            ast::expr::Expr::Typed { ty, item: expr } => self.with_meta(Expr::Typed {
                ty: self.gen_type(ty)?,
                item: Box::new(self.gen(expr)?),
            }),
            ast::expr::Expr::Function { parameters, body } => {
                let (node_id, span) = self.pop_span().unwrap();
                parameters
                    .iter()
                    .map(|parameter| self.gen_type(parameter))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .try_rfold(self.gen(body)?, |body, parameter| {
                        self.push_span(node_id.clone(), span.clone());
                        Ok(self.with_meta(Expr::Function {
                            parameter,
                            body: Box::new(body),
                        }))
                    })?
            }
            ast::expr::Expr::Vector(items) => self.with_meta(Expr::Vector(
                items
                    .iter()
                    .map(|item| self.gen(item))
                    .collect::<Result<_, _>>()?,
            )),
            ast::expr::Expr::Set(items) => self.with_meta(Expr::Set(
                items
                    .iter()
                    .map(|item| self.gen(item))
                    .collect::<Result<_, _>>()?,
            )),
            ast::expr::Expr::Import { ty: _, uuid: _ } => todo!(),
            ast::expr::Expr::Export { ty: _ } => todo!(),
            ast::expr::Expr::Attribute { attr, item: expr } => {
                self.pop_span();
                let mut ret = self.gen(expr)?;
                let attr = self.gen(attr)?.value;
                ret.meta.attrs.push(attr);
                self.attrs
                    .borrow_mut()
                    .insert(ret.id.clone(), ret.meta.attrs.clone());
                ret
            }
            ast::expr::Expr::Brand { brands, item: expr } => {
                brands.iter().for_each(|brand| {
                    self.brands.borrow_mut().insert(brand.clone());
                });
                self.gen(expr)?
            }
            ast::expr::Expr::Match { of, cases } => self.with_meta(Expr::Match {
                of: Box::new(self.gen(of)?),
                cases: cases
                    .iter()
                    .map(|ast::expr::MatchCase { ty, expr }| {
                        Ok(MatchCase {
                            ty: self.gen_type(ty)?,
                            expr: self.gen(expr)?,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            ast::expr::Expr::Label { label, item: expr } => {
                if self.brands.borrow().contains(label) {
                    self.with_meta(Expr::Brand {
                        brand: label.clone(),
                        item: Box::new(self.gen(expr)?),
                    })
                } else {
                    self.with_meta(Expr::Label {
                        label: label.clone(),
                        item: Box::new(self.gen(expr)?),
                    })
                }
            }
            ast::expr::Expr::NewType { ident, ty, expr } => {
                let ty = self.gen_type(ty)?.value;
                self.type_aliases.borrow_mut().insert(ident.clone(), ty);
                self.gen(expr)?
            }
            ast::expr::Expr::Comment { item, .. } => self.gen(item)?,
            ast::expr::Expr::Card { uuid, .. } => {
                return Err(HirGenError::UnexpectedCard { ident: *uuid });
            }
        };
        Ok(with_meta)
    }

    pub fn gen_hir(&mut self, ast: &WithSpan<ast::expr::Expr>) -> Result<(), HirGenError> {
        self.entrypoint = Some(self.gen(ast)?);
        Ok(())
    }

    pub(crate) fn push_span(&self, node_id: NodeId, span: Span) {
        self.next_span.borrow_mut().push((node_id, span));
    }

    pub(crate) fn pop_span(&self) -> Option<(NodeId, Span)> {
        self.next_span.borrow_mut().pop()
    }

    pub fn next_id(&self) -> usize {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() += 1;
        id
    }

    fn with_meta<T: std::fmt::Debug>(&self, value: T) -> WithMeta<T> {
        let span = self.pop_span().unwrap();
        WithMeta {
            id: span.0,
            meta: Meta {
                attrs: vec![],
                file_id: self.file_id.clone(),
                // no span is a bug of hirgen, so unwrap is safe
                span: Some(span.1),
            },
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::span::dummy_span;
    use hir::{
        helper::remove_meta,
        meta::{dummy_meta, Meta},
        ty::Type,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    fn parse(input: &str) -> WithSpan<ast::expr::Expr> {
        parser::parse(lexer::scan(input).unwrap()).unwrap()
    }

    #[test]
    fn test() {
        let gen = HirGen::default();
        assert_eq!(
            remove_meta(
                gen.gen(&dummy_span(ast::expr::Expr::Apply {
                    function: dummy_span(ast::ty::Type::Number),
                    link_name: Default::default(),
                    arguments: vec![dummy_span(ast::expr::Expr::Attribute {
                        attr: Box::new(dummy_span(ast::expr::Expr::Literal(
                            ast::expr::Literal::Integer(1)
                        ),)),
                        item: Box::new(dummy_span(ast::expr::Expr::Attribute {
                            attr: Box::new(dummy_span(ast::expr::Expr::Literal(
                                ast::expr::Literal::Integer(2)
                            ),)),
                            item: Box::new(dummy_span(ast::expr::Expr::Hole)),
                        },)),
                    },)],
                },),)
                    .unwrap()
            ),
            WithMeta {
                id: Default::default(),
                meta: Meta::default(),
                value: Expr::Apply {
                    function: WithMeta {
                        id: Default::default(),
                        meta: Meta::default(),
                        value: Type::Number
                    },
                    link_name: Default::default(),
                    arguments: vec![WithMeta {
                        id: Default::default(),
                        meta: Meta {
                            attrs: vec![
                                Expr::Literal(Literal::Integer(2)),
                                Expr::Literal(Literal::Integer(1))
                            ],
                            file_id: Default::default(),
                            span: Default::default()
                        },
                        value: Expr::Literal(Literal::Hole)
                    }],
                },
            }
        );

        assert_eq!(gen.attrs.borrow().len(), 1);
        assert_eq!(
            gen.attrs.borrow().iter().next().unwrap().1,
            &vec![
                Expr::Literal(Literal::Integer(2)),
                Expr::Literal(Literal::Integer(1))
            ]
        );
    }

    #[test]
    fn label_and_brand() {
        let expr = parse(
            r#"
        $ & @brand 'number ~
        'brand brand ~
        $ & @brand 'number ~
        & @label 'number
        "#,
        );

        let gen = HirGen::default();
        assert_eq!(
            remove_meta(gen.gen(&expr).unwrap()),
            dummy_meta(Expr::Let {
                ty: dummy_meta(Type::Infer),
                definition: Box::new(dummy_meta(Expr::Apply {
                    function: dummy_meta(Type::Label {
                        label: "brand".into(),
                        item: Box::new(dummy_meta(Type::Number)),
                    }),
                    link_name: Default::default(),
                    arguments: vec![],
                })),
                expression: Box::new(dummy_meta(Expr::Let {
                    ty: dummy_meta(Type::Infer),
                    definition: Box::new(dummy_meta(Expr::Apply {
                        function: dummy_meta(Type::Brand {
                            brand: "brand".into(),
                            item: Box::new(dummy_meta(Type::Number)),
                        }),
                        link_name: Default::default(),
                        arguments: vec![],
                    })),
                    expression: Box::new(dummy_meta(Expr::Apply {
                        function: dummy_meta(Type::Label {
                            label: "label".into(),
                            item: Box::new(dummy_meta(Type::Number)),
                        }),
                        link_name: Default::default(),
                        arguments: vec![],
                    }))
                }))
            })
        )
    }

    #[test]
    fn gen_entrypoint() {
        let expr = parse(
            r#"
        1
        "#,
        );
        let (_, hir) = gen_hir(&expr).unwrap();
        assert!(hir.cards.is_empty());
        assert_eq!(
            remove_meta(hir.entrypoint.unwrap()),
            dummy_meta(Expr::Literal(Literal::Integer(1)))
        );
    }
}
