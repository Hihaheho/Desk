use std::str::FromStr;
use uuid::Uuid;

use crate::{
    grammar_trait::{
        self, ApplyGroup, ApplyGroupParam, ApplyGroupParams, CommentBlockComment,
        CommentInlineComment, EffectExprAddEffects, EffectExprApplyEffects, EffectExprEffects,
        EffectExprSubEffects, ExprApply, ExprAttributed, ExprBrand, ExprCard, ExprCast,
        ExprCommentExpr, ExprContinue, ExprDo, ExprExists, ExprExprBeginExprExprEnd, ExprForall,
        ExprFunction, ExprHandle, ExprHole, ExprLabeled, ExprLet, ExprLiteral, ExprMap, ExprMatch,
        ExprNewType, ExprPerform, ExprProduct, ExprReference, ExprVector, IdentIdentWrapped,
        Integer, LinkNameCardKeyUuid, LinkNameVersionKeyUuid, LiteralInteger, LiteralRational,
        LiteralReal, LiteralString, TyAttributedTy, TyCommentTy, TyEffectful, TyExistsTy,
        TyExprBeginTyExprEnd, TyForallTy, TyFunctionTy, TyInfer, TyIntegerKey, TyLabeledTy,
        TyLetTy, TyMapTy, TyProductTy, TyRationalKey, TyRealKey, TyStringKey, TySum, TyThis,
        TyTrait, TyVariable, TyVecTy,
    },
    MinimalistSyntaxError,
};
use ast::{
    expr::{Expr, Handler, Literal, MapElem, MatchCase},
    span::WithSpan,
    ty::{Effect, Function},
};
use dson::Dson;

use ids::{CardId, LinkName, NodeId};
impl TryFrom<grammar_trait::Expr<'_>> for WithSpan<Expr> {
    type Error = MinimalistSyntaxError;
    fn try_from(expr: grammar_trait::Expr) -> Result<Self, Self::Error> {
        let expr = match expr {
            grammar_trait::Expr::CommentExpr(ExprCommentExpr { comment, expr }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Comment {
                    text: (*comment).into(),
                    item: Box::new((*expr).try_into()?),
                },
            },
            grammar_trait::Expr::ExprBeginExprExprEnd(ExprExprBeginExprExprEnd { expr }) => {
                (*expr).try_into()?
            }
            grammar_trait::Expr::Hole(ExprHole { hole: _ }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Hole,
            },
            grammar_trait::Expr::Do(ExprDo { r#do }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Do {
                    stmt: Box::new((*r#do.expr).try_into()?),
                    expr: Box::new((*r#do.expr0).try_into()?),
                },
            },
            grammar_trait::Expr::Cast(ExprCast { cast }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Typed {
                    ty: (*cast.ty).try_into()?,
                    item: Box::new((*cast.expr).try_into()?),
                },
            },
            grammar_trait::Expr::Literal(ExprLiteral { literal }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Literal(match *literal {
                    grammar_trait::Literal::Rational(LiteralRational { rational }) => {
                        let text = rational.rational.text().to_string();
                        let splitted: Vec<_> = text.split("/").collect();
                        let a = splitted[0].trim_end().parse().map_err(|_| {
                            MinimalistSyntaxError::OtherError(format!(
                                "Could not parse numerator of rational: {}",
                                rational.rational.text()
                            ))
                        })?;
                        let b = splitted[1].trim_start().parse().map_err(|_| {
                            MinimalistSyntaxError::OtherError(format!(
                                "Could not parse denominator of rational: {}",
                                rational.rational.text()
                            ))
                        })?;
                        Literal::Rational(a, b)
                    }
                    grammar_trait::Literal::Integer(LiteralInteger { integer }) => {
                        let integer = match *integer {
                            Integer::Hex(integer) => i64::from_str_radix(
                                integer.hex.hex.text().to_string().get(2..).unwrap(),
                                16,
                            ),
                            Integer::Oct(integer) => i64::from_str_radix(
                                integer.oct.oct.text().to_string().get(2..).unwrap(),
                                8,
                            ),
                            Integer::Bin(integer) => i64::from_str_radix(
                                integer.bin.bin.text().to_string().get(2..).unwrap(),
                                2,
                            ),
                            Integer::Dec(integer) => integer.dec.dec.text().parse::<i64>(),
                        }
                        .map_err(|_| {
                            MinimalistSyntaxError::OtherError(format!("Could not parse integer",))
                        })?;
                        Literal::Integer(integer)
                    }
                    grammar_trait::Literal::Real(LiteralReal { real }) => Literal::Real(
                        real.real.text().to_string().parse::<f64>().map_err(|_| {
                            MinimalistSyntaxError::OtherError(format!(
                                "Could not parse real: {}",
                                real.real.text().to_string()
                            ))
                        })?,
                    ),
                    grammar_trait::Literal::String(LiteralString { string }) => {
                        let string = string
                            .string_list
                            .into_iter()
                            .map(|s| match *s.string_list_group {
                                grammar_trait::StringListGroup::Escaped(escaped) => {
                                    match escaped.escaped.escaped.text().to_string().pop().unwrap()
                                    {
                                        't' => "\t",
                                        'n' => "\n",
                                        '"' => "\"",
                                        '\\' => "\\",
                                        other => panic!("invalid escape sequence {}", other),
                                    }
                                    .into()
                                }
                                grammar_trait::StringListGroup::Characters(other) => {
                                    other.characters.characters.text().to_string()
                                }
                                grammar_trait::StringListGroup::Newlines(newline) => {
                                    newline.newlines.newlines.text().to_string()
                                }
                            })
                            .collect::<String>();
                        Literal::String(string)
                    }
                }),
            },
            grammar_trait::Expr::Let(ExprLet { r#let }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Let {
                    definition: Box::new((*r#let.expr).try_into()?),
                    body: Box::new((*r#let.expr0).try_into()?),
                },
            },
            grammar_trait::Expr::Perform(ExprPerform { perform }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Perform {
                    input: Box::new((*perform.expr).try_into()?),
                    output: (*perform.ty).try_into()?,
                },
            },
            grammar_trait::Expr::Continue(ExprContinue { r#continue }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Continue {
                    input: Box::new((*r#continue.expr).try_into()?),
                    output: (*r#continue.ty).try_into()?,
                },
            },
            grammar_trait::Expr::Handle(ExprHandle { handle }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Handle {
                    expr: Box::new((*handle.expr).try_into()?),
                    handlers: handle
                        .handle_list
                        .into_iter()
                        .map(|handler| {
                            Ok(Handler {
                                effect: Effect {
                                    input: (*handler.handler.ty).try_into()?,
                                    output: (*handler.handler.ty0).try_into()?,
                                },
                                handler: (*handler.handler.expr).try_into()?,
                            })
                        })
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                },
            },
            grammar_trait::Expr::Product(ExprProduct { product }) => WithSpan {
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
            grammar_trait::Expr::Vector(ExprVector { vector }) => WithSpan {
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
            grammar_trait::Expr::Map(ExprMap { map }) => WithSpan {
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
            grammar_trait::Expr::Attributed(ExprAttributed { attributed }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Attributed {
                    attr: (*attributed.attribute.expr).try_into()?,
                    item: Box::new((*attributed.expr).try_into()?),
                },
            },
            grammar_trait::Expr::Match(ExprMatch { r#match }) => WithSpan {
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
            grammar_trait::Expr::Function(ExprFunction { function }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Function {
                    parameter: (*function.ty).try_into()?,
                    body: Box::new((*function.expr).try_into()?),
                },
            },
            grammar_trait::Expr::Apply(ExprApply { apply }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Apply {
                    function: (*apply.ty).try_into()?,
                    link_name: try_into_link_name(apply.apply_opt.map(|opt| *opt.link_name))?,
                    arguments: match *apply.apply_group {
                        ApplyGroup::Param(ApplyGroupParam { param }) => {
                            vec![(*param.expr).try_into()?]
                        }
                        ApplyGroup::Params(ApplyGroupParams { params }) => params
                            .params_list
                            .into_iter()
                            .map(|param| (*param.expr).try_into())
                            .collect::<Result<_, _>>()?,
                    },
                },
            },
            grammar_trait::Expr::Reference(ExprReference { reference }) => WithSpan {
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
            grammar_trait::Expr::Labeled(ExprLabeled { labeled }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Label {
                    label: (*labeled.label.expr).try_into()?,
                    item: Box::new((*labeled.expr).try_into()?),
                },
            },
            grammar_trait::Expr::Forall(ExprForall { .. }) => todo!(),
            grammar_trait::Expr::Exists(ExprExists { .. }) => todo!(),
            grammar_trait::Expr::NewType(ExprNewType { new_type }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::NewType {
                    ident: (*new_type.ident).into(),
                    ty: (*new_type.ty).try_into()?,
                    expr: Box::new((*new_type.expr).try_into()?),
                },
            },
            grammar_trait::Expr::Card(ExprCard { card }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: Expr::Card {
                    id: CardId((*card.uuid).try_into()?),
                    item: Box::new((*card.expr).try_into()?),
                    next: Box::new((*card.expr0).try_into()?),
                },
            },
            grammar_trait::Expr::Brand(ExprBrand { brand }) => WithSpan {
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
            grammar_trait::Ty::ExprBeginTyExprEnd(TyExprBeginTyExprEnd { ty }) => {
                (*ty).try_into()?
            }
            grammar_trait::Ty::Infer(TyInfer { infer: _ }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Infer,
            },
            grammar_trait::Ty::This(TyThis { this: _ }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::This,
            },
            grammar_trait::Ty::RealKey(TyRealKey { .. }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Real,
            },
            grammar_trait::Ty::RationalKey(TyRationalKey { .. }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Rational,
            },
            grammar_trait::Ty::IntegerKey(TyIntegerKey { .. }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Integer,
            },
            grammar_trait::Ty::StringKey(TyStringKey { string_key: _ }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::String,
            },
            grammar_trait::Ty::Effectful(TyEffectful { effectful }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Effectful {
                    ty: Box::new((*effectful.ty).try_into()?),
                    effects: (*effectful.effect_expr).try_into()?,
                },
            },
            grammar_trait::Ty::CommentTy(TyCommentTy { comment, ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Comment {
                    text: (*comment).into(),
                    item: Box::new((*ty).try_into()?),
                },
            },
            grammar_trait::Ty::Trait(TyTrait { r#trait }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Trait(
                    r#trait
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
            },
            grammar_trait::Ty::ProductTy(TyProductTy { product_ty }) => WithSpan {
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
            grammar_trait::Ty::Sum(TySum { sum }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Sum(
                    sum.sum_list
                        .into_iter()
                        .map(|t| (*t.ty).try_into())
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                ),
            },
            grammar_trait::Ty::VecTy(TyVecTy { vec_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Vector(Box::new((*vec_ty.ty).try_into()?)),
            },
            grammar_trait::Ty::MapTy(TyMapTy { map_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Map {
                    key: Box::new((*map_ty.ty).try_into()?),
                    value: Box::new((*map_ty.ty0).try_into()?),
                },
            },
            grammar_trait::Ty::FunctionTy(TyFunctionTy { function_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Function(Box::new(Function {
                    parameter: (*function_ty.ty).try_into()?,
                    body: (*function_ty.ty0).try_into()?,
                })),
            },
            grammar_trait::Ty::LabeledTy(TyLabeledTy { labeled_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Labeled {
                    brand: (*labeled_ty.label.expr).try_into()?,
                    item: Box::new((*labeled_ty.ty).try_into()?),
                },
            },
            grammar_trait::Ty::AttributedTy(TyAttributedTy { attributed_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Attributed {
                    attr: (*attributed_ty.attribute.expr).try_into()?,
                    ty: Box::new((*attributed_ty.ty).try_into()?),
                },
            },
            grammar_trait::Ty::Variable(TyVariable { variable }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Variable((*variable.ident).into()),
            },
            grammar_trait::Ty::LetTy(TyLetTy { let_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Let {
                    variable: (*let_ty.ident).into(),
                    definition: Box::new((*let_ty.ty).try_into()?),
                    body: Box::new((*let_ty.ty0).try_into()?),
                },
            },
            grammar_trait::Ty::ForallTy(TyForallTy { forall_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Forall {
                    variable: (*forall_ty.bounded_variable.ident).into(),
                    bound: forall_ty
                        .bounded_variable
                        .bounded_variable_opt
                        .map::<Result<_, MinimalistSyntaxError>, _>(|trait_| {
                            Ok(Box::new((*trait_.ty).try_into()?))
                        })
                        .transpose()?,
                    body: Box::new((*forall_ty.ty).try_into()?),
                },
            },
            grammar_trait::Ty::ExistsTy(TyExistsTy { exists_ty }) => WithSpan {
                id: NodeId::new(),
                span: 0..0,
                value: ast::ty::Type::Exists {
                    variable: (*exists_ty.bounded_variable.ident).into(),
                    bound: exists_ty
                        .bounded_variable
                        .bounded_variable_opt
                        .map::<Result<_, MinimalistSyntaxError>, _>(|trait_| {
                            Ok(Box::new((*trait_.ty).try_into()?))
                        })
                        .transpose()?,
                    body: Box::new((*exists_ty.ty).try_into()?),
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
            grammar_trait::EffectExpr::Effects(EffectExprEffects { effects }) => WithSpan {
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
            grammar_trait::EffectExpr::AddEffects(EffectExprAddEffects { add_effects }) => {
                WithSpan {
                    id: NodeId::new(),
                    span: 0..0,
                    value: ast::ty::EffectExpr::Add(
                        add_effects
                            .add_effects_list
                            .into_iter()
                            .map(|add_effect| (*add_effect.effect_expr).try_into())
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                }
            }
            grammar_trait::EffectExpr::SubEffects(EffectExprSubEffects { sub_effects }) => {
                WithSpan {
                    id: NodeId::new(),
                    span: 0..0,
                    value: ast::ty::EffectExpr::Sub {
                        minuend: Box::new((*sub_effects.effect_expr).try_into()?),
                        subtrahend: Box::new((*sub_effects.effect_expr0).try_into()?),
                    },
                }
            }
            grammar_trait::EffectExpr::ApplyEffects(EffectExprApplyEffects { apply_effects }) => {
                WithSpan {
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
                }
            }
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
    fn from(ident: grammar_trait::Ident<'_>) -> Self {
        match ident {
            grammar_trait::Ident::IdentRaw(raw) => raw.ident_raw.ident_raw.text().to_string(),
            grammar_trait::Ident::IdentWrapped(IdentIdentWrapped { ident_wrapped }) => {
                [*ident_wrapped.ident_part]
                    .into_iter()
                    .chain(
                        ident_wrapped
                            .ident_wrapped_list
                            .into_iter()
                            .map(|list| *list.ident_part),
                    )
                    .map(|part| part.ident_part.text().to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        }
    }
}

impl TryFrom<grammar_trait::Trait<'_>> for Vec<WithSpan<Function>> {
    type Error = MinimalistSyntaxError;

    fn try_from(value: grammar_trait::Trait<'_>) -> Result<Self, Self::Error> {
        Ok(value
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
            .collect::<Result<_, MinimalistSyntaxError>>()?)
    }
}

impl From<grammar_trait::Comment<'_>> for String {
    fn from(comment: grammar_trait::Comment) -> Self {
        match comment {
            grammar_trait::Comment::BlockComment(CommentBlockComment { block_comment }) => {
                block_comment
                    .block_comment_list
                    .into_iter()
                    .map(|content| {
                        content
                            .block_comment_content
                            .block_comment_content
                            .text()
                            .to_string()
                    })
                    .collect::<String>()
                    + &block_comment
                        .block_comment_list0
                        .into_iter()
                        .map(|_| ")")
                        .collect::<String>()
            }
            grammar_trait::Comment::InlineComment(CommentInlineComment { inline_comment }) => {
                inline_comment
                    .comment_characters
                    .comment_characters
                    .to_string()
            }
        }
    }
}

fn try_into_link_name(
    value: Option<grammar_trait::LinkName<'_>>,
) -> Result<LinkName, MinimalistSyntaxError> {
    match value {
        Some(grammar_trait::LinkName::CardKeyUuid(LinkNameCardKeyUuid { uuid })) => {
            Ok(LinkName::Card((*uuid).try_into()?))
        }
        Some(grammar_trait::LinkName::VersionKeyUuid(LinkNameVersionKeyUuid { uuid })) => {
            Ok(LinkName::Version((*uuid).try_into()?))
        }
        None => Ok(LinkName::None),
    }
}
