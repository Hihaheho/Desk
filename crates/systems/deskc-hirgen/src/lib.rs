mod error;
mod gen_effect_expr;
use dson::Dson;
use ids::{CardId, NodeId};

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use ast::span::{Span, WithSpan};
use error::HirGenError;
use hir::{
    expr::{Expr, Handler, Literal, MapElem, MatchCase},
    meta::{Meta, WithMeta},
    ty::{Function, Type},
    Hir,
};

pub fn gen_cards(src: &WithSpan<ast::expr::Expr>) -> Result<(HirGen, Hir), HirGenError> {
    let mut hirgen = HirGen::default();
    hirgen.gen_hir(src)?;
    let hir = Hir {
        expr: hirgen.entrypoint.take(),
        cards: hirgen.cards.drain(..).collect(),
    };
    Ok((hirgen, hir))
}

pub fn gen_hir(src: &WithSpan<ast::expr::Expr>) -> Result<(HirGen, WithMeta<Expr>), HirGenError> {
    let hirgen = HirGen::default();
    let card = hirgen.gen_card(src)?;
    Ok((hirgen, card))
}

#[derive(Default, Debug)]
pub struct HirGen {
    next_id: RefCell<usize>,
    next_span: RefCell<Vec<(NodeId, Span)>>,
    pub attrs: RefCell<HashMap<NodeId, Vec<Dson>>>,
    pub type_aliases: RefCell<HashMap<String, Type>>,
    brands: RefCell<HashSet<Dson>>,
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
            ast::ty::Type::Trait(trait_) => self.with_meta(Type::Trait(
                trait_
                    .iter()
                    .map(|function| {
                        self.push_span(function.id.clone(), function.span.clone());
                        Ok(self.with_meta(Function {
                            parameter: self.gen_type(&function.value.parameter)?,
                            body: self.gen_type(&function.value.body)?,
                        }))
                    })
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
            ast::ty::Type::Function(function) => {
                self.with_meta(Type::Function(Box::new(Function {
                    parameter: self.gen_type(&function.parameter)?,
                    body: self.gen_type(&function.body)?,
                })))
            }
            ast::ty::Type::Vector(ty) => self.with_meta(Type::Vector(Box::new(self.gen_type(ty)?))),
            ast::ty::Type::Map { key, value } => self.with_meta(Type::Map {
                key: Box::new(self.gen_type(key)?),
                value: Box::new(self.gen_type(value)?),
            }),
            ast::ty::Type::Let {
                variable,
                definition,
                body,
            } => self.with_meta(Type::Let {
                variable: variable.clone(),
                definition: Box::new(self.gen_type(definition)?),
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
            ast::ty::Type::Attributed { attr, ty } => {
                self.pop_span();
                let mut ret = self.gen_type(ty)?;
                ret.meta.attrs.push(attr.clone());
                self.attrs
                    .borrow_mut()
                    .insert(ret.id.clone(), ret.meta.attrs.clone());
                ret
            }
            ast::ty::Type::Comment { item, .. } => self.gen_type(item)?,
            ast::ty::Type::Forall {
                variable,
                bound,
                body,
            } => self.with_meta(Type::Forall {
                variable: variable.clone(),
                bound: bound
                    .as_ref()
                    .map(|bound| Ok(Box::new(self.gen_type(&bound)?)))
                    .transpose()?,
                body: Box::new(self.gen_type(body)?),
            }),
            ast::ty::Type::Exists {
                variable,
                bound,
                body,
            } => self.with_meta(Type::Exists {
                variable: variable.clone(),
                bound: bound
                    .as_ref()
                    .map(|bound| Ok(Box::new(self.gen_type(&bound)?)))
                    .transpose()?,
                body: Box::new(self.gen_type(body)?),
            }),
        };
        Ok(with_meta)
    }

    pub fn gen_card(&self, ast: &WithSpan<ast::expr::Expr>) -> Result<WithMeta<Expr>, HirGenError> {
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
            ast::expr::Expr::Do { stmt, expr } => self.with_meta(Expr::Do {
                stmt: Box::new(self.gen_card(stmt)?),
                expr: Box::new(self.gen_card(expr)?),
            }),
            ast::expr::Expr::Let {
                definition,
                body: expression,
            } => self.with_meta(Expr::Let {
                definition: Box::new(self.gen_card(definition)?),
                expression: Box::new(self.gen_card(expression)?),
            }),
            ast::expr::Expr::Perform { input, output } => self.with_meta(Expr::Perform {
                input: Box::new(self.gen_card(input)?),
                output: self.gen_type(output)?,
            }),
            ast::expr::Expr::Continue { input, output } => self.with_meta(Expr::Continue {
                input: Box::new(self.gen_card(input)?),
                output: self.gen_type(output)?,
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
                                handler: self.gen_card(handler)?,
                            })
                        },
                    )
                    .collect::<Result<Vec<_>, _>>()?,
                expr: Box::new(self.gen_card(expr)?),
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
                    .map(|argument| self.gen_card(argument))
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            ast::expr::Expr::Product(items) => self.with_meta(Expr::Product(
                items
                    .iter()
                    .map(|item| self.gen_card(item))
                    .collect::<Result<_, _>>()?,
            )),
            ast::expr::Expr::Typed { ty, item: expr } => self.with_meta(Expr::Typed {
                ty: self.gen_type(ty)?,
                item: Box::new(self.gen_card(expr)?),
            }),
            ast::expr::Expr::Function { parameter, body } => self.with_meta(Expr::Function {
                parameter: self.gen_type(parameter)?,
                body: Box::new(self.gen_card(body)?),
            }),
            ast::expr::Expr::Vector(items) => self.with_meta(Expr::Vector(
                items
                    .iter()
                    .map(|item| self.gen_card(item))
                    .collect::<Result<_, _>>()?,
            )),
            ast::expr::Expr::Map(elems) => self.with_meta(Expr::Map(
                elems
                    .iter()
                    .map(|elem| {
                        Ok(MapElem {
                            key: self.gen_card(&elem.key)?,
                            value: self.gen_card(&elem.value)?,
                        })
                    })
                    .collect::<Result<_, _>>()?,
            )),
            ast::expr::Expr::Import { ty: _, uuid: _ } => todo!(),
            ast::expr::Expr::Export { ty: _ } => todo!(),
            ast::expr::Expr::Attributed { attr, item: expr } => {
                self.pop_span();
                let mut ret = self.gen_card(expr)?;
                ret.meta.attrs.push(attr.clone());
                self.attrs
                    .borrow_mut()
                    .insert(ret.id.clone(), ret.meta.attrs.clone());
                ret
            }
            ast::expr::Expr::Brand { brand, item: expr } => {
                self.brands.borrow_mut().insert(brand.clone());
                self.gen_card(expr)?
            }
            ast::expr::Expr::Match { of, cases } => self.with_meta(Expr::Match {
                of: Box::new(self.gen_card(of)?),
                cases: cases
                    .iter()
                    .map(|ast::expr::MatchCase { ty, expr }| {
                        Ok(MatchCase {
                            ty: self.gen_type(ty)?,
                            expr: self.gen_card(expr)?,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            ast::expr::Expr::Label { label, item: expr } => {
                if self.brands.borrow().contains(label) {
                    self.with_meta(Expr::Brand {
                        brand: label.clone(),
                        item: Box::new(self.gen_card(expr)?),
                    })
                } else {
                    self.with_meta(Expr::Label {
                        label: label.clone(),
                        item: Box::new(self.gen_card(expr)?),
                    })
                }
            }
            ast::expr::Expr::NewType { ident, ty, expr } => {
                let ty = self.gen_type(ty)?.value;
                self.type_aliases.borrow_mut().insert(ident.clone(), ty);
                self.gen_card(expr)?
            }
            ast::expr::Expr::Comment { item, .. } => self.gen_card(item)?,
            ast::expr::Expr::Card { uuid, .. } => {
                return Err(HirGenError::UnexpectedCard { ident: *uuid });
            }
        };
        Ok(with_meta)
    }

    pub fn gen_hir(&mut self, ast: &WithSpan<ast::expr::Expr>) -> Result<(), HirGenError> {
        self.entrypoint = Some(self.gen_card(ast)?);
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
                // no span is a bug of hirgen, so unwrap is safe
                span: Some(span.1),
            },
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ast::span::dummy_span;
    use deskc::card::CardQueries;
    use deskc::{Code, SyntaxKind};
    use hir::{
        helper::remove_meta,
        meta::{dummy_meta, Meta},
        ty::Type,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    fn parse(input: &str) -> Arc<WithSpan<ast::expr::Expr>> {
        let card_id = CardId::new();
        let mut compiler = deskc::card::CardsCompiler::default();
        compiler.set_code(
            card_id.clone(),
            Code::SourceCode {
                syntax: SyntaxKind::Minimalist,
                source: Arc::new(input.to_string()),
            },
        );
        compiler.ast(card_id).unwrap()
    }

    #[test]
    fn test() {
        let gen = HirGen::default();
        assert_eq!(
            remove_meta(
                gen.gen_card(&dummy_span(ast::expr::Expr::Apply {
                    function: dummy_span(ast::ty::Type::Number),
                    link_name: Default::default(),
                    arguments: vec![dummy_span(ast::expr::Expr::Attributed {
                        attr: Dson::Literal(dson::Literal::Integer(1)),
                        item: Box::new(dummy_span(ast::expr::Expr::Attributed {
                            attr: Dson::Literal(dson::Literal::Integer(2)),
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
                                Dson::Literal(dson::Literal::Integer(2)),
                                Dson::Literal(dson::Literal::Integer(1)),
                            ],
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
                Dson::Literal(dson::Literal::Integer(2)),
                Dson::Literal(dson::Literal::Integer(1)),
            ]
        );
    }

    #[test]
    fn label_and_brand() {
        let expr = parse(
            r#"
        $ & @"brand" 'number;
        'brand "brand";
        $ & @"brand" 'number;
        & @"label" 'number
        "#,
        );

        let gen = HirGen::default();
        assert_eq!(
            remove_meta(gen.gen_card(&expr).unwrap()),
            dummy_meta(Expr::Let {
                definition: Box::new(dummy_meta(Expr::Apply {
                    function: dummy_meta(Type::Label {
                        label: Dson::Literal(dson::Literal::String("brand".into())),
                        item: Box::new(dummy_meta(Type::Number)),
                    }),
                    link_name: Default::default(),
                    arguments: vec![],
                })),
                expression: Box::new(dummy_meta(Expr::Let {
                    definition: Box::new(dummy_meta(Expr::Apply {
                        function: dummy_meta(Type::Brand {
                            brand: Dson::Literal(dson::Literal::String("brand".into())),
                            item: Box::new(dummy_meta(Type::Number)),
                        }),
                        link_name: Default::default(),
                        arguments: vec![],
                    })),
                    expression: Box::new(dummy_meta(Expr::Apply {
                        function: dummy_meta(Type::Label {
                            label: Dson::Literal(dson::Literal::String("label".into())),
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
        let expr = parse("1");
        let (_, hir) = gen_cards(&expr).unwrap();
        assert!(hir.cards.is_empty());
        assert_eq!(
            remove_meta(hir.expr.unwrap()),
            dummy_meta(Expr::Literal(Literal::Integer(1)))
        );
    }
}
