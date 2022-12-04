use std::str::FromStr;
use uuid::Uuid;

use crate::grammar_trait::{
    Comment, EffectExpr0, EffectExpr1, EffectExpr2, EffectExpr3, Expr0, Expr1, Expr10, Expr11,
    Expr12, Expr13, Expr14, Expr15, Expr16, Expr17, Expr18, Expr19, Expr2, Expr20, Expr3, Expr4,
    Expr5, Expr6, Expr7, Expr8, Expr9, Integer, LinkName0, LinkName1, Literal0, Literal1, Literal2,
    Literal3, Ty0, Ty1, Ty10, Ty11, Ty12, Ty13, Ty14, Ty15, Ty16, Ty17, Ty2, Ty3, Ty4, Ty5, Ty6,
    Ty7, Ty8, Ty9,
};
use crate::{grammar_trait, MinimalistSyntaxError};
use ast::{
    expr::{Expr, Handler, Literal, MapElem, MatchCase},
    span::WithSpan,
    ty::{Function, Trait},
};
use dson::Dson;

use ids::{LinkName, NodeId};
impl TryFrom<grammar_trait::Expr<'_>> for WithSpan<Expr> {
    type Error = MinimalistSyntaxError;
    fn try_from(expr: grammar_trait::Expr) -> Result<Self, Self::Error> {
        let expr = match expr {
            grammar_trait::Expr::Expr0(Expr0 { comment, expr }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Comment {
                    text: (*comment).into(),
                    item: Box::new((*expr).try_into()?),
                },
            },
            grammar_trait::Expr::Expr1(Expr1 { hole: _ }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Hole,
            },
            grammar_trait::Expr::Expr2(Expr2 { r#do }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Do {
                    stmt: Box::new((*r#do.expr).try_into()?),
                    expr: Box::new((*r#do.expr0).try_into()?),
                },
            },
            grammar_trait::Expr::Expr3(Expr3 { cast }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Typed {
                    ty: (*cast.ty).try_into()?,
                    item: Box::new((*cast.expr).try_into()?),
                },
            },
            grammar_trait::Expr::Expr4(Expr4 { literal }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Literal(match *literal {
                    grammar_trait::Literal::Literal0(Literal0 { integer }) => {
                        let string = match *integer {
                            Integer::Integer0(integer) => integer.dec.dec.text().to_string(),
                            Integer::Integer1(integer) => integer.hex.hex.text().to_string(),
                            Integer::Integer2(integer) => integer.oct.oct.text().to_string(),
                            Integer::Integer3(integer) => integer.bin.bin.text().to_string(),
                        };
                        Literal::Integer(string.parse().map_err(|_| {
                            MinimalistSyntaxError::ParseError(format!(
                                "Could not parse integer: {}",
                                string
                            ))
                        })?)
                    }
                    grammar_trait::Literal::Literal1(Literal1 { float }) => {
                        Literal::Float(float.float.to_string().parse().map_err(|_| {
                            MinimalistSyntaxError::ParseError(format!(
                                "Could not parse float: {}",
                                float.float.text().to_string()
                            ))
                        })?)
                    }
                    grammar_trait::Literal::Literal2(Literal2 { rational }) => {
                        let rational = rational
                            .rational
                            .to_string()
                            .split("/")
                            .map(|s| {
                                str::parse(s).map_err(|_| {
                                    MinimalistSyntaxError::ParseError(format!(
                                        "Could not parse rational: {}",
                                        rational.rational.text().to_string()
                                    ))
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Literal::Rational(rational[0], rational[1])
                    }
                    grammar_trait::Literal::Literal3(Literal3 { string }) => {
                        let string = string
                            .string_list
                            .into_iter()
                            .map(|s| match *s.string_list_group {
                                grammar_trait::StringListGroup::StringListGroup0(escaped) => {
                                    escaped
                                        .l_bracket_n_r_bracket
                                        .text()
                                        .to_string()
                                        .pop()
                                        .unwrap()
                                        .to_string()
                                }
                                grammar_trait::StringListGroup::StringListGroup1(other) => {
                                    other.l_bracket_circumflex_r_bracket_star.text().to_string()
                                }
                                grammar_trait::StringListGroup::StringListGroup2(newline) => {
                                    newline.n_star.text().to_string()
                                }
                            })
                            .collect::<String>();
                        Literal::String(string)
                    }
                }),
            },
            grammar_trait::Expr::Expr5(Expr5 { r#let }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Let {
                    definition: Box::new((*r#let.expr).try_into()?),
                    body: Box::new((*r#let.expr0).try_into()?),
                },
            },
            grammar_trait::Expr::Expr6(Expr6 { perform }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Perform {
                    input: Box::new((*perform.expr).try_into()?),
                    output: perform
                        .perform_opt
                        .map(|output| (*output.ty).try_into())
                        .transpose()?,
                },
            },
            grammar_trait::Expr::Expr7(Expr7 { r#continue }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Continue {
                    input: Box::new((*r#continue.expr).try_into()?),
                    output: r#continue
                        .continue_opt
                        .map(|output| (*output.ty).try_into())
                        .transpose()?,
                },
            },
            grammar_trait::Expr::Expr8(Expr8 { handle }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Handle {
                    expr: Box::new((*handle.expr).try_into()?),
                    handlers: handle
                        .handle_list
                        .into_iter()
                        .map(|handler| {
                            Ok(Handler {
                                input: (*handler.handler.ty).try_into()?,
                                output: (*handler.handler.ty0).try_into()?,
                                handler: (*handler.handler.expr).try_into()?,
                            })
                        })
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                },
            },
            grammar_trait::Expr::Expr9(Expr9 { product }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Product(
                    product
                        .product_list
                        .into_iter()
                        .map(|product| (*product.expr).try_into())
                        .collect::<Result<_, _>>()?,
                ),
            },
            grammar_trait::Expr::Expr10(Expr10 { vector }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Vector(
                    vector
                        .vector_list
                        .into_iter()
                        .map(|vector| (*vector.expr).try_into())
                        .collect::<Result<_, _>>()?,
                ),
            },
            grammar_trait::Expr::Expr11(Expr11 { map }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Map(
                    map.map_list
                        .into_iter()
                        .map(|map| {
                            Ok(MapElem {
                                key: (*map.expr).try_into()?,
                                value: (*map.expr0).try_into()?,
                            })
                        })
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                ),
            },
            grammar_trait::Expr::Expr12(Expr12 { attributed }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Attributed {
                    attr: (*attributed.attribute.expr).try_into()?,
                    item: Box::new((*attributed.expr).try_into()?),
                },
            },
            grammar_trait::Expr::Expr13(Expr13 { r#match }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Match {
                    of: Box::new((*r#match.expr).try_into()?),
                    cases: r#match
                        .match_list
                        .into_iter()
                        .map(|r#match| {
                            Ok(MatchCase {
                                ty: (*r#match.case.ty).try_into()?,
                                expr: (*r#match.case.expr).try_into()?,
                            })
                        })
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                },
            },
            grammar_trait::Expr::Expr14(Expr14 { function }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Function {
                    parameter: (*function.ty).try_into()?,
                    body: Box::new((*function.expr).try_into()?),
                },
            },
            grammar_trait::Expr::Expr15(Expr15 { apply }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Apply {
                    function: (*apply.ty).try_into()?,
                    link_name: try_into_link_name(apply.apply_opt.map(|opt| *opt.link_name))?,
                    arguments: apply
                        .apply_list
                        .into_iter()
                        .map(|arg| (*arg.expr).try_into())
                        .collect::<Result<_, _>>()?,
                },
            },
            grammar_trait::Expr::Expr16(Expr16 { reference }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Apply {
                    function: (*reference.ty).try_into()?,
                    link_name: try_into_link_name(
                        reference.reference_opt.map(|opt| *opt.link_name),
                    )?,
                    arguments: vec![],
                },
            },
            grammar_trait::Expr::Expr17(Expr17 { labeled }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Label {
                    label: (*labeled.label.expr).try_into()?,
                    item: Box::new((*labeled.expr).try_into()?),
                },
            },
            grammar_trait::Expr::Expr18(Expr18 { new_type }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::NewType {
                    ident: (*new_type.ident).into(),
                    ty: (*new_type.ty).try_into()?,
                    expr: Box::new((*new_type.expr).try_into()?),
                },
            },
            grammar_trait::Expr::Expr19(Expr19 { card }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Card {
                    uuid: (*card.uuid).try_into()?,
                    item: Box::new((*card.expr).try_into()?),
                    next: Some(Box::new((*card.expr0).try_into()?)),
                },
            },
            grammar_trait::Expr::Expr20(Expr20 { brand }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Brand {
                    brand: (*brand.expr).try_into()?,
                    item: Box::new((*brand.expr0).try_into()?),
                },
            },
        };
        Ok(expr)
    }
}

impl TryFrom<grammar_trait::Ty<'_>> for WithSpan<ast::ty::Type> {
    type Error = MinimalistSyntaxError;
    fn try_from(ty: grammar_trait::Ty) -> Result<Self, Self::Error> {
        let ty = match ty {
            grammar_trait::Ty::Ty0(Ty0 { infer: _ }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Infer,
            },
            grammar_trait::Ty::Ty1(Ty1 { this: _ }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::This,
            },
            grammar_trait::Ty::Ty2(Ty2 { number_key: _ }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Number,
            },
            grammar_trait::Ty::Ty3(Ty3 { string_key: _ }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::String,
            },
            grammar_trait::Ty::Ty4(Ty4 { effectful }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Effectful {
                    ty: Box::new((*effectful.ty).try_into()?),
                    effects: (*effectful.effect_expr).try_into()?,
                },
            },
            grammar_trait::Ty::Ty5(Ty5 { comment, ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Comment {
                    text: (*comment).into(),
                    item: Box::new((*ty).try_into()?),
                },
            },
            grammar_trait::Ty::Ty6(Ty6 { r#trait }) => {
                let WithSpan { id, span, value } = (*r#trait).try_into()?;
                WithSpan {
                    id,
                    span,
                    value: ast::ty::Type::Trait(value),
                }
            }
            grammar_trait::Ty::Ty7(Ty7 { product_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Product(
                    product_ty
                        .product_ty_list
                        .into_iter()
                        .map(|t| (*t.ty).try_into())
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                ),
            },
            grammar_trait::Ty::Ty8(Ty8 { sum }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Sum(
                    sum.sum_list
                        .into_iter()
                        .map(|t| (*t.ty).try_into())
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                ),
            },
            grammar_trait::Ty::Ty9(Ty9 { vec_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Vector(Box::new((*vec_ty.ty).try_into()?)),
            },
            grammar_trait::Ty::Ty10(Ty10 { map_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Map {
                    key: Box::new((*map_ty.ty).try_into()?),
                    value: Box::new((*map_ty.ty0).try_into()?),
                },
            },
            grammar_trait::Ty::Ty11(Ty11 { function_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Function(Box::new(Function {
                    parameter: (*function_ty.ty).try_into()?,
                    body: (*function_ty.ty0).try_into()?,
                })),
            },
            grammar_trait::Ty::Ty12(Ty12 { labeled_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Brand {
                    brand: (*labeled_ty.label.expr).try_into()?,
                    item: Box::new((*labeled_ty.ty).try_into()?),
                },
            },
            grammar_trait::Ty::Ty13(Ty13 { attributed_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Attributed {
                    attr: (*attributed_ty.attribute.expr).try_into()?,
                    ty: Box::new((*attributed_ty.ty).try_into()?),
                },
            },
            grammar_trait::Ty::Ty14(Ty14 { variable }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Variable((*variable.ident).into()),
            },
            grammar_trait::Ty::Ty15(Ty15 { let_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Let {
                    variable: (*let_ty.ident).into(),
                    definition: Box::new((*let_ty.ty).try_into()?),
                    body: Box::new((*let_ty.ty0).try_into()?),
                },
            },
            grammar_trait::Ty::Ty16(Ty16 { all }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Forall {
                    variable: (*all.ident).into(),
                    bound: all
                        .all_opt
                        .map::<Result<_, MinimalistSyntaxError>, _>(|trait_| {
                            Ok(Box::new((*trait_.ty).try_into()?))
                        })
                        .transpose()?,
                    body: Box::new((*all.ty).try_into()?),
                },
            },
            grammar_trait::Ty::Ty17(Ty17 { exist }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Exists {
                    variable: (*exist.ident).into(),
                    bound: exist
                        .exist_opt
                        .map::<Result<_, MinimalistSyntaxError>, _>(|trait_| {
                            Ok(Box::new((*trait_.ty).try_into()?))
                        })
                        .transpose()?,
                    body: Box::new((*exist.ty).try_into()?),
                },
            },
        };
        Ok(ty)
    }
}

impl TryFrom<grammar_trait::EffectExpr<'_>> for WithSpan<ast::ty::EffectExpr> {
    type Error = MinimalistSyntaxError;
    fn try_from(effect_expr: grammar_trait::EffectExpr) -> Result<Self, Self::Error> {
        let effect_expr = match effect_expr {
            grammar_trait::EffectExpr::EffectExpr0(EffectExpr0 { effects }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::EffectExpr::Effects(
                    effects
                        .effects_list
                        .into_iter()
                        .map(|effect| (*effect.effect).try_into())
                        .collect::<Result<Vec<_>, _>>()?,
                ),
            },
            grammar_trait::EffectExpr::EffectExpr1(EffectExpr1 { add_effects }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::EffectExpr::Add(
                    add_effects
                        .add_effects_list
                        .into_iter()
                        .map(|add_effect| (*add_effect.effect_expr).try_into())
                        .collect::<Result<Vec<_>, _>>()?,
                ),
            },
            grammar_trait::EffectExpr::EffectExpr2(EffectExpr2 { sub_effects }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::EffectExpr::Sub {
                    minuend: Box::new((*sub_effects.effect_expr).try_into()?),
                    subtrahend: Box::new((*sub_effects.effect_expr0).try_into()?),
                },
            },
            grammar_trait::EffectExpr::EffectExpr3(EffectExpr3 { apply_effects }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::EffectExpr::Apply {
                    function: Box::new((*apply_effects.ty).try_into()?),
                    arguments: apply_effects
                        .apply_effects_list
                        .into_iter()
                        .map(|apply_effect| (*apply_effect.ty).try_into())
                        .collect::<Result<Vec<_>, _>>()?,
                },
            },
        };
        Ok(effect_expr)
    }
}

impl TryFrom<grammar_trait::Effect<'_>> for WithSpan<ast::ty::Effect> {
    type Error = MinimalistSyntaxError;
    fn try_from(effect: grammar_trait::Effect) -> Result<Self, Self::Error> {
        Ok(WithSpan {
            id: NodeId::new(),
            span: 0..0,
            value: ast::ty::Effect {
                input: (*effect.ty).try_into()?,
                output: (*effect.ty0).try_into()?,
            },
        })
    }
}

impl TryFrom<grammar_trait::Expr<'_>> for Dson {
    type Error = MinimalistSyntaxError;

    fn try_from(value: grammar_trait::Expr<'_>) -> Result<Self, Self::Error> {
        let expr: WithSpan<Expr> = value.try_into()?;
        expr.try_into()
            .map_err(|err| MinimalistSyntaxError::DsonError(err))
    }
}

impl TryFrom<grammar_trait::Uuid<'_>> for Uuid {
    type Error = MinimalistSyntaxError;

    fn try_from(value: grammar_trait::Uuid<'_>) -> Result<Self, Self::Error> {
        let value = value.uuid_text.uuid_text.text().to_string();
        Uuid::from_str(&value).map_err(|e| MinimalistSyntaxError::UuidError(e))
    }
}

impl From<grammar_trait::Ident<'_>> for String {
    fn from(value: grammar_trait::Ident<'_>) -> Self {
        match value {
            grammar_trait::Ident::Ident0(ident) => {
                ident.ident_no_space.ident_no_space.text().to_string()
            }
            // TODO: remove backtick and repeated whitespace
            grammar_trait::Ident::Ident1(ident) => {
                ident.ident_wrapped.ident_wrapped.text().to_string()
            }
        }
    }
}

impl TryFrom<grammar_trait::Trait<'_>> for WithSpan<Trait> {
    type Error = MinimalistSyntaxError;

    fn try_from(value: grammar_trait::Trait<'_>) -> Result<Self, Self::Error> {
        Ok(WithSpan {
            id: NodeId::new(),
            span: 0..0,
            value: Trait(
                value
                    .trait_list
                    .into_iter()
                    .map(|t| {
                        Ok(WithSpan {
                            id: NodeId::new(),
                            span: 0..0,
                            value: Function {
                                parameter: (*t.function_ty.ty).try_into()?,
                                body: (*t.function_ty.ty0).try_into()?,
                            },
                        })
                    })
                    .collect::<Result<_, MinimalistSyntaxError>>()?,
            ),
        })
    }
}

impl From<Comment<'_>> for String {
    fn from(comment: Comment) -> Self {
        match comment {
            grammar_trait::Comment::Comment0(comment) => comment
                .block_comment_content
                .block_comment_content
                .to_string(),
            grammar_trait::Comment::Comment1(comment) => comment.dot_star.to_string(),
        }
    }
}

fn try_into_link_name(
    value: Option<grammar_trait::LinkName<'_>>,
) -> Result<LinkName, MinimalistSyntaxError> {
    match value {
        Some(grammar_trait::LinkName::LinkName0(LinkName0 { card_key: _, uuid })) => {
            Ok(LinkName::Card((*uuid).try_into()?))
        }
        Some(grammar_trait::LinkName::LinkName1(LinkName1 { uuid })) => {
            Ok(LinkName::Version((*uuid).try_into()?))
        }
        None => Ok(LinkName::None),
    }
}
