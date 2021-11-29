mod error;
mod extract_includes;
pub use extract_includes::extract_includes;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use ast::span::{Span, Spanned};
use error::HirGenError;
use file::{FileId, InFile};
use hir::{
    expr::{Expr, Literal, MatchCase},
    meta::{Id, Meta, WithMeta},
    ty::{Effect, Type},
};

pub fn gen_hir(
    file_id: FileId,
    src: &Spanned<ast::expr::Expr>,
    included: HashMap<String, InFile<Spanned<ast::expr::Expr>>>,
) -> Result<(HirGen, WithMeta<Expr>), HirGenError> {
    let hir = HirGen {
        file_stack: RefCell::new(vec![file_id]),
        included,
        ..Default::default()
    };
    hir.gen(src).map(|expr| (hir, expr))
}

#[derive(Default)]
pub struct HirGen {
    next_id: RefCell<Id>,
    next_span: RefCell<Vec<Span>>,
    pub variables: RefCell<HashMap<String, Id>>,
    pub attrs: RefCell<HashMap<Id, Vec<Expr>>>,
    pub included: HashMap<String, InFile<Spanned<ast::expr::Expr>>>,
    pub type_aliases: RefCell<HashMap<String, Type>>,
    brands: RefCell<HashSet<String>>,
    // current file id is the last item.
    file_stack: RefCell<Vec<FileId>>,
}

impl HirGen {
    pub fn push_file_id(&self, file_id: FileId) {
        self.file_stack.borrow_mut().push(file_id);
    }
    pub fn pop_file_id(&self) -> FileId {
        self.file_stack.borrow_mut().pop().unwrap()
    }
    pub fn gen_type(&self, ty: &Spanned<ast::ty::Type>) -> Result<WithMeta<Type>, HirGenError> {
        let (ty, span) = ty;
        self.push_span(span.clone());

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
                ty: Box::new(self.gen_type(ty)?),
                effects: effects
                    .into_iter()
                    .map(|effect| self.effect(effect))
                    .collect::<Result<_, _>>()?,
            }),
            ast::ty::Type::Infer => self.with_meta(Type::Infer),
            ast::ty::Type::This => self.with_meta(Type::This),
            ast::ty::Type::Alias(alias) => self.with_meta(
                self.type_aliases
                    .borrow()
                    .get(alias)
                    .cloned()
                    .ok_or_else(|| HirGenError::UnknownTypeAlias {
                        alias: alias.clone(),
                        span: self.pop_span().unwrap(),
                    })?,
            ),
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
                    .try_rfold(self.gen_type(body)?, |body, parameter| {
                        self.push_span(span.clone());
                        Ok(self.with_meta(Type::Function {
                            parameter: Box::new(parameter),
                            body: Box::new(body),
                        }))
                    })?
            }
            ast::ty::Type::Array(ty) => self.with_meta(Type::Array(Box::new(self.gen_type(ty)?))),
            ast::ty::Type::Set(ty) => self.with_meta(Type::Set(Box::new(self.gen_type(ty)?))),
            ast::ty::Type::Let { definition, body } => self.with_meta(Type::Let {
                definition: Box::new(self.gen_type(definition)?),
                body: Box::new(self.gen_type(body)?),
            }),
            ast::ty::Type::Variable(ident) => {
                self.with_meta(Type::Variable(self.get_id_of(ident.clone())))
            }
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
                if let Some(meta) = &mut ret.meta {
                    let attr = self.gen(attr)?.value;
                    meta.attrs.push(attr);
                    self.attrs.borrow_mut().insert(meta.id, meta.attrs.clone());
                }
                ret
            }
        };
        Ok(with_meta)
    }

    pub fn gen(&self, ast: &Spanned<ast::expr::Expr>) -> Result<WithMeta<Expr>, HirGenError> {
        let (expr, span) = ast;
        self.push_span(span.clone());

        let with_meta = match expr {
            ast::expr::Expr::Literal(literal) => self.with_meta(Expr::Literal(match literal {
                ast::expr::Literal::String(value) => Literal::String(value.clone()),
                ast::expr::Literal::Int(value) => Literal::Int(*value),
                ast::expr::Literal::Rational(a, b) => Literal::Rational(*a, *b),
                ast::expr::Literal::Float(value) => Literal::Float(*value),
            })),
            ast::expr::Expr::Hole => self.with_meta(Expr::Literal(Literal::Hole)),
            ast::expr::Expr::Let {
                ty: variable,
                definition,
                expression,
            } => self.with_meta(Expr::Let {
                ty: self.gen_type(variable)?,
                definition: Box::new(self.gen(definition)?),
                expression: Box::new(self.gen(expression)?),
            }),
            ast::expr::Expr::Perform { input, output } => self.with_meta(Expr::Perform {
                input: Box::new(self.gen(input)?),
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
                handler: Box::new(self.gen(handler)?),
                expr: Box::new(self.gen(expr)?),
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
            ast::expr::Expr::Typed { ty, item: expr } => self.with_meta(Expr::Typed {
                ty: self.gen_type(ty)?,
                item: Box::new(self.gen(expr)?),
            }),
            ast::expr::Expr::Function { parameters, body } => {
                let span = self.pop_span().unwrap();
                parameters
                    .into_iter()
                    .map(|parameter| self.gen_type(parameter))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .try_rfold(self.gen(body)?, |body, parameter| {
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
            ast::expr::Expr::Include(file) => {
                let InFile { id, expr } = self.included.get(file).unwrap();
                self.push_file_id(*id);
                let ret = self.gen(expr)?;
                self.pop_file_id();
                ret
            }
            ast::expr::Expr::Import { ty, uuid } => todo!(),
            ast::expr::Expr::Export { ty } => todo!(),
            ast::expr::Expr::Attribute { attr, item: expr } => {
                self.pop_span();
                let mut ret = self.gen(expr)?;
                if let Some(meta) = &mut ret.meta {
                    let attr = self.gen(attr)?.value;
                    meta.attrs.push(attr.clone());
                    self.attrs.borrow_mut().insert(meta.id, meta.attrs.clone());
                }
                ret
            }
            ast::expr::Expr::Brand { brands, item: expr } => {
                brands.into_iter().for_each(|brand| {
                    self.brands.borrow_mut().insert(brand.clone());
                });
                self.gen(expr)?
            }
            ast::expr::Expr::Match { of, cases } => self.with_meta(Expr::Match {
                of: Box::new(self.gen(of)?),
                cases: cases
                    .into_iter()
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
                self.type_aliases
                    .borrow_mut()
                    .insert(ident.clone(), self.gen_type(ty)?.value);
                self.gen(expr)?
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

    fn get_id_of(&self, ident: String) -> usize {
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

    fn with_meta<T: std::fmt::Debug>(&self, value: T) -> WithMeta<T> {
        WithMeta {
            meta: Some(Meta {
                attrs: vec![],
                id: self.next_id(),
                file_id: self.file_stack.borrow().last().unwrap().clone(),
                span: self.pop_span().unwrap(),
            }),
            value,
        }
    }

    fn effect(
        &self,
        ast::ty::Effect { input, output }: &ast::ty::Effect,
    ) -> Result<Effect, HirGenError> {
        Ok(Effect {
            input: self.gen_type(input)?,
            output: self.gen_type(output)?,
        })
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
        parser::parse(lexer::scan(input).unwrap()).unwrap()
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
            Expr::Typed { ty, item: expr } => Expr::Typed {
                ty: remove_meta_ty(ty),
                item: Box::new(remove_meta(*expr)),
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
        no_meta(value)
    }

    #[test]
    fn test() {
        let gen = HirGen::default();
        gen.push_file_id(FileId(0));
        assert_eq!(
            gen.gen(&(
                ast::expr::Expr::Apply {
                    function: (ast::ty::Type::Number, 3..10),
                    arguments: vec![(
                        ast::expr::Expr::Attribute {
                            attr: Box::new((
                                ast::expr::Expr::Literal(ast::expr::Literal::Int(1)),
                                24..25
                            )),
                            item: Box::new((
                                ast::expr::Expr::Attribute {
                                    attr: Box::new((
                                        ast::expr::Expr::Literal(ast::expr::Literal::Int(2)),
                                        24..25
                                    )),
                                    item: Box::new((ast::expr::Expr::Hole, 26..27)),
                                },
                                25..26
                            )),
                        },
                        24..27
                    )],
                },
                0..27
            ),),
            Ok(WithMeta {
                meta: Some(Meta {
                    attrs: vec![],
                    id: 4,
                    file_id: FileId(0),
                    span: 0..27
                }),
                value: Expr::Apply {
                    function: WithMeta {
                        meta: Some(Meta {
                            attrs: vec![],
                            id: 0,
                            file_id: FileId(0),
                            span: 3..10
                        }),
                        value: Type::Number
                    },
                    arguments: vec![WithMeta {
                        meta: Some(Meta {
                            attrs: vec![
                                Expr::Literal(Literal::Int(2)),
                                Expr::Literal(Literal::Int(1))
                            ],
                            id: 1,
                            file_id: FileId(0),
                            span: 26..27
                        }),
                        value: Expr::Literal(Literal::Hole)
                    }],
                },
            })
        );

        assert_eq!(
            gen.attrs.borrow_mut().get(&1),
            Some(&vec![
                Expr::Literal(Literal::Int(2)),
                Expr::Literal(Literal::Int(1))
            ])
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
        gen.push_file_id(FileId(0));
        assert_eq!(
            remove_meta(gen.gen(&expr).unwrap()),
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
