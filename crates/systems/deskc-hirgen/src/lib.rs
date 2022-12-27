mod error;
mod gen_effect_expr;

use dson::Dson;
use ids::NodeId;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use ast::span::{Span, WithSpan};
use error::HirGenError;
use hir::{
    expr::{Expr, Handler, Literal, MapElem, MatchCase},
    meta::{Meta, WithMeta},
    ty::{Effect, Function, Type},
    Card, Cards,
};

pub fn gen_cards(src: &WithSpan<ast::expr::Expr>) -> Result<(HirGen, Cards), HirGenError> {
    let hirgen = HirGen::default();
    let file = hirgen.gen_cards(src)?;
    let hir = Cards {
        cards: hirgen.cards.borrow_mut().drain(..).collect(),
        file,
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
    cards: RefCell<Vec<Card>>,
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
            ast::ty::Type::Real => self.with_meta(Type::Real),
            ast::ty::Type::Rational => self.with_meta(Type::Rational),
            ast::ty::Type::Integer => self.with_meta(Type::Integer),
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
            ast::ty::Type::Labeled { brand, item } => {
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
                    .insert(ret.meta.id.clone(), ret.meta.attrs.clone());
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
                    .map(|bound| Ok(Box::new(self.gen_type(bound)?)))
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
                    .map(|bound| Ok(Box::new(self.gen_type(bound)?)))
                    .transpose()?,
                body: Box::new(self.gen_type(body)?),
            }),
        };
        Ok(with_meta)
    }

    pub fn gen_cards(
        &self,
        ast: &WithSpan<ast::expr::Expr>,
    ) -> Result<WithMeta<Expr>, HirGenError> {
        match &ast.value {
            ast::expr::Expr::Card { id, item, next } => {
                self.cards.borrow_mut().push(Card {
                    id: id.clone(),
                    hir: self.gen_card(item)?,
                });
                let next = self.gen_cards(next)?;
                Ok(next)
            }
            ast::expr::Expr::Comment { item, .. } => self.gen_cards(item),
            ast::expr::Expr::NewType { ident, ty, expr } => {
                self.add_new_type(ty, ident)?;
                self.gen_cards(expr)
            }
            _ => self.gen_card(ast),
        }
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
                ast::expr::Literal::Real(value) => Literal::Real(*value),
            })),
            ast::expr::Expr::Hole => self.with_meta(Expr::Perform {
                input: Box::new(self.with_no_span(Expr::Product(vec![]))),
                output: self.with_no_span(Type::Infer),
            }),
            ast::expr::Expr::Do { stmt, expr } => self.with_meta(Expr::Do {
                stmt: Box::new(self.gen_card(stmt)?),
                expr: Box::new(self.gen_card(expr)?),
            }),
            ast::expr::Expr::Let {
                definition,
                body: expression,
            } => self.with_meta(Expr::Let {
                definition: Box::new(self.gen_card(definition)?),
                expr: Box::new(self.gen_card(expression)?),
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
                    .map(|ast::expr::Handler { effect, handler }| {
                        Ok(Handler {
                            effect: Effect {
                                input: self.gen_type(&effect.value.input)?,
                                output: self.gen_type(&effect.value.output)?,
                            },
                            handler: self.gen_card(handler)?,
                        })
                    })
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
            ast::expr::Expr::Attributed { attr, item: expr } => {
                self.pop_span();
                let mut ret = self.gen_card(expr)?;
                ret.meta.attrs.push(attr.clone());
                self.attrs
                    .borrow_mut()
                    .insert(ret.meta.id.clone(), ret.meta.attrs.clone());
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
                self.add_new_type(ty, ident)?;
                self.gen_card(expr)?
            }
            ast::expr::Expr::Comment { item, .. } => self.gen_card(item)?,
            ast::expr::Expr::Card { id, .. } => {
                return Err(HirGenError::UnexpectedCard {
                    card_id: id.clone(),
                });
            }
        };
        Ok(with_meta)
    }

    pub(crate) fn add_new_type(
        &self,
        ty: &WithSpan<ast::ty::Type>,
        ident: &str,
    ) -> Result<(), HirGenError> {
        let ty = self.gen_type(ty)?.value;
        self.type_aliases.borrow_mut().insert(ident.to_string(), ty);
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
            meta: Meta {
                id: span.0,
                attrs: vec![],
                // no span is a bug of hirgen, so unwrap is safe
                span: Some(span.1),
            },
            value,
        }
    }

    fn with_no_span<T: std::fmt::Debug>(&self, value: T) -> WithMeta<T> {
        WithMeta {
            meta: Meta {
                id: NodeId::new(),
                attrs: vec![],
                span: None,
            },
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ast::span::dummy_span;
    use deskc::card::DeskcQueries;
    use deskc::{Code, SyntaxKind};
    use hir::{
        meta::{dummy_meta, Meta},
        ty::Type,
        visitor::remove_meta,
    };
    use ids::FileId;
    use pretty_assertions::assert_eq;

    use super::*;

    fn parse(input: &str) -> Arc<WithSpan<ast::expr::Expr>> {
        let file_id = FileId::new();
        let mut compiler = deskc::card::DeskCompiler::default();
        compiler.set_code(
            file_id.clone(),
            Code::SourceCode {
                syntax: SyntaxKind::Minimalist,
                source: Arc::new(input.to_string()),
            },
        );
        compiler.ast(file_id).unwrap()
    }

    #[test]
    fn test() {
        let gen = HirGen::default();
        assert_eq!(
            remove_meta(
                gen.gen_card(&dummy_span(ast::expr::Expr::Apply {
                    function: dummy_span(ast::ty::Type::Real),
                    link_name: Default::default(),
                    arguments: vec![dummy_span(ast::expr::Expr::Attributed {
                        attr: Dson::Literal(dson::Literal::Integer(1)),
                        item: Box::new(dummy_span(ast::expr::Expr::Attributed {
                            attr: Dson::Literal(dson::Literal::Integer(2)),
                            item: Box::new(dummy_span(ast::expr::Expr::Literal(
                                ast::expr::Literal::Integer(3)
                            ))),
                        },)),
                    },)],
                },),)
                    .unwrap()
            ),
            WithMeta {
                meta: Meta::default(),
                value: Expr::Apply {
                    function: WithMeta {
                        meta: Meta::default(),
                        value: Type::Real
                    },
                    link_name: Default::default(),
                    arguments: vec![WithMeta {
                        meta: Meta {
                            id: Default::default(),
                            attrs: vec![
                                Dson::Literal(dson::Literal::Integer(2)),
                                Dson::Literal(dson::Literal::Integer(1)),
                            ],
                            span: Default::default()
                        },
                        value: Expr::Literal(Literal::Integer(3))
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
        $ & @"brand" 'integer;
        'brand "brand";
        $ & @"brand" 'integer;
        & @"label" 'integer
        "#,
        );

        let gen = HirGen::default();
        assert_eq!(
            remove_meta(gen.gen_card(&expr).unwrap()),
            dummy_meta(Expr::Let {
                definition: Box::new(dummy_meta(Expr::Apply {
                    function: dummy_meta(Type::Label {
                        label: Dson::Literal(dson::Literal::String("brand".into())),
                        item: Box::new(dummy_meta(Type::Integer)),
                    }),
                    link_name: Default::default(),
                    arguments: vec![],
                })),
                expr: Box::new(dummy_meta(Expr::Let {
                    definition: Box::new(dummy_meta(Expr::Apply {
                        function: dummy_meta(Type::Brand {
                            brand: Dson::Literal(dson::Literal::String("brand".into())),
                            item: Box::new(dummy_meta(Type::Integer)),
                        }),
                        link_name: Default::default(),
                        arguments: vec![],
                    })),
                    expr: Box::new(dummy_meta(Expr::Apply {
                        function: dummy_meta(Type::Label {
                            label: Dson::Literal(dson::Literal::String("label".into())),
                            item: Box::new(dummy_meta(Type::Integer)),
                        }),
                        link_name: Default::default(),
                        arguments: vec![],
                    }))
                }))
            })
        )
    }

    #[test]
    fn gen_rest() {
        let expr = parse("1");
        let (_, hir) = gen_cards(&expr).unwrap();
        assert!(hir.cards.is_empty());
        assert_eq!(
            remove_meta(hir.file),
            dummy_meta(Expr::Literal(Literal::Integer(1)))
        );
    }
}
