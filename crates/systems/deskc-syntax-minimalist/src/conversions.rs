use std::str::FromStr;
use uuid::Uuid;

use crate::{
    grammar_trait::{
        self, ApplyGroup, ApplyGroupParam, ApplyGroupParams, CommentBlockComment,
        CommentInlineComment, EffectExprAddEffects, EffectExprApplyEffects, EffectExprEffects,
        EffectExprSubEffects, ExprApply, ExprAttributed, ExprBrand, ExprCard, ExprCast,
        ExprContinue, ExprDo, ExprExists, ExprExprBeginExprCExprEnd, ExprForall, ExprFunction,
        ExprHandle, ExprHole, ExprLabeled, ExprLet, ExprLiteral, ExprMap, ExprMatch, ExprNewType,
        ExprPerform, ExprProduct, ExprReference, ExprVector, IdentIdentWrapped, Integer,
        LinkNameCardKeyUuid, LinkNameVersionKeyUuid, LiteralInteger, LiteralRational,
        LiteralRawString, LiteralReal, LiteralString, RawString, TyAttributedTy, TyEffectful,
        TyExistsTy, TyExprBeginTyExprEnd, TyForallTy, TyFunctionTy, TyInfer, TyIntegerKey,
        TyLabeledTy, TyLetTy, TyMapTy, TyProductTy, TyRationalKey, TyRealKey, TyStringKey, TySum,
        TyVariable, TyVecTy,
    },
    MinimalistSyntaxError,
};
use ast::{
    expr::{Expr, Handler, Literal, MapElem, MatchCase},
    meta::{Meta, WithMeta},
    ty::{Effect, Function},
};
use dson::Dson;

use ids::{CardId, LinkName, NodeId};
impl TryFrom<grammar_trait::ExprC<'_>> for WithMeta<Expr> {
    type Error = MinimalistSyntaxError;
    fn try_from(expr: grammar_trait::ExprC) -> Result<Self, Self::Error> {
        let comments = expr
            .expr_c_list
            .into_iter()
            .map(|comment| *comment.comment)
            .map(Into::into)
            .collect::<Vec<_>>()
            .into();
        let expr = match *expr.expr {
            grammar_trait::Expr::ExprBeginExprCExprEnd(ExprExprBeginExprCExprEnd { expr_c }) => {
                (*expr_c).try_into()?
            }
            grammar_trait::Expr::Hole(ExprHole { hole: _ }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Hole,
            },
            grammar_trait::Expr::Do(ExprDo { r#do }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Do {
                    stmt: Box::new((*r#do.expr_c).try_into()?),
                    expr: Box::new((*r#do.expr_c0).try_into()?),
                },
            },
            grammar_trait::Expr::Cast(ExprCast { cast }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Typed {
                    ty: (*cast.ty).try_into()?,
                    item: Box::new((*cast.expr_c).try_into()?),
                },
            },
            grammar_trait::Expr::Literal(ExprLiteral { literal }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Literal(match *literal {
                    grammar_trait::Literal::Rational(LiteralRational { rational }) => {
                        let text = rational.rational.text().to_string();
                        let splitted: Vec<_> = text.split('/').collect();
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
                        let integer = match &*integer {
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
                            MinimalistSyntaxError::OtherError(format!(
                                "Could not parse integer {integer:?}"
                            ))
                        })?;
                        Literal::Integer(integer)
                    }
                    grammar_trait::Literal::Real(LiteralReal { real }) => {
                        Literal::Real(real.real.text().parse::<f64>().map_err(|_| {
                            MinimalistSyntaxError::OtherError(format!(
                                "Could not parse real: {}",
                                real.real.text().to_string()
                            ))
                        })?)
                    }
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
                                        other => panic!("invalid escape sequence {other}"),
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
                    grammar_trait::Literal::RawString(LiteralRawString { raw_string }) => {
                        let string = match *raw_string {
                            RawString::RawString1(raw_string) => raw_string
                                .raw_string1
                                .raw_string1_opt
                                .map(|token| {
                                    token.raw_characters1.raw_characters1.text().to_string()
                                })
                                .unwrap_or_default(),
                            RawString::RawString2(raw_string) => raw_string
                                .raw_string2
                                .raw_string2_opt
                                .map(|token| {
                                    token.raw_characters2.raw_characters2.text().to_string()
                                })
                                .unwrap_or_default(),
                        };
                        Literal::String(string)
                    }
                }),
            },
            grammar_trait::Expr::Let(ExprLet { r#let }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Let {
                    definition: Box::new((*r#let.expr_c).try_into()?),
                    body: Box::new((*r#let.expr_c0).try_into()?),
                },
            },
            grammar_trait::Expr::Perform(ExprPerform { perform }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Perform {
                    input: Box::new((*perform.expr_c).try_into()?),
                    output: (*perform.ty).try_into()?,
                },
            },
            grammar_trait::Expr::Continue(ExprContinue { r#continue }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Continue {
                    input: Box::new((*r#continue.expr_c).try_into()?),
                    output: (*r#continue.ty).try_into()?,
                },
            },
            grammar_trait::Expr::Handle(ExprHandle { handle }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Handle {
                    expr: Box::new((*handle.expr_c).try_into()?),
                    handlers: handle
                        .handle_list
                        .into_iter()
                        .map(|handler| {
                            let comments = handler
                                .handler
                                .handler_list
                                .into_iter()
                                .map(|handler_list| *handler_list.comment)
                                .map(Into::into)
                                .collect::<Vec<_>>()
                                .into();
                            Ok(WithMeta {
                                meta: Meta {
                                    id: NodeId::new(),
                                    comments,
                                },
                                value: Handler {
                                    effect: WithMeta {
                                        meta: Meta {
                                            id: NodeId::new(),
                                            comments: vec![].into(),
                                        },
                                        value: Effect {
                                            input: (*handler.handler.ty).try_into()?,
                                            output: (*handler.handler.ty0).try_into()?,
                                        },
                                    },
                                    handler: (*handler.handler.expr_c).try_into()?,
                                },
                            })
                        })
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                },
            },
            grammar_trait::Expr::Product(ExprProduct { product }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Product(
                    product
                        .product_list
                        .into_iter()
                        .map(|product| (*product.expr_c).try_into())
                        .collect::<Result<_, _>>()?,
                ),
            },
            grammar_trait::Expr::Vector(ExprVector { vector }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Vector(
                    vector
                        .vector_list
                        .into_iter()
                        .map(|vector| (*vector.expr_c).try_into())
                        .collect::<Result<_, _>>()?,
                ),
            },
            grammar_trait::Expr::Map(ExprMap { map }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Map(
                    map.map_list
                        .into_iter()
                        .map(|map| {
                            let comments = vec![].into();
                            Ok(WithMeta {
                                meta: Meta {
                                    id: NodeId::new(),
                                    comments,
                                },
                                value: MapElem {
                                    key: (*map.expr_c).try_into()?,
                                    value: (*map.expr_c0).try_into()?,
                                },
                            })
                        })
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                ),
            },
            grammar_trait::Expr::Attributed(ExprAttributed { attributed }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Attributed {
                    attr: (*attributed.attribute.expr_c).try_into()?,
                    item: Box::new((*attributed.expr_c).try_into()?),
                },
            },
            grammar_trait::Expr::Match(ExprMatch { r#match }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Match {
                    of: Box::new((*r#match.expr_c).try_into()?),
                    cases: r#match
                        .match_list
                        .into_iter()
                        .map(|r#match| {
                            let comments = r#match
                                .case
                                .case_list
                                .into_iter()
                                .map(|case_list| *case_list.comment)
                                .map(Into::into)
                                .collect::<Vec<_>>()
                                .into();
                            Ok(WithMeta {
                                meta: Meta {
                                    id: NodeId::new(),
                                    comments,
                                },
                                value: MatchCase {
                                    ty: (*r#match.case.ty).try_into()?,
                                    expr: (*r#match.case.expr_c).try_into()?,
                                },
                            })
                        })
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                },
            },
            grammar_trait::Expr::Function(ExprFunction { function }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Function {
                    parameter: (*function.ty).try_into()?,
                    body: Box::new((*function.expr_c).try_into()?),
                },
            },
            grammar_trait::Expr::Apply(ExprApply { apply }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Apply {
                    function: (*apply.ty).try_into()?,
                    link_name: try_into_link_name(apply.apply_opt.map(|opt| *opt.link_name))?,
                    arguments: match *apply.apply_group {
                        ApplyGroup::Param(ApplyGroupParam { param }) => {
                            vec![(*param.expr_c).try_into()?]
                        }
                        ApplyGroup::Params(ApplyGroupParams { params }) => params
                            .params_list
                            .into_iter()
                            .map(|param| (*param.expr_c).try_into())
                            .collect::<Result<_, _>>()?,
                    },
                },
            },
            grammar_trait::Expr::Reference(ExprReference { reference }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Apply {
                    function: (*reference.ty).try_into()?,
                    link_name: try_into_link_name(
                        reference.reference_opt.map(|opt| *opt.link_name),
                    )?,
                    arguments: vec![],
                },
            },
            grammar_trait::Expr::Labeled(ExprLabeled { labeled }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Label {
                    label: (*labeled.label.ident).into(),
                    item: Box::new((*labeled.expr_c).try_into()?),
                },
            },
            grammar_trait::Expr::Forall(ExprForall { .. }) => todo!(),
            grammar_trait::Expr::Exists(ExprExists { .. }) => todo!(),
            grammar_trait::Expr::NewType(ExprNewType { new_type }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::NewType {
                    ident: (*new_type.ident).into(),
                    ty: (*new_type.ty).try_into()?,
                    expr: Box::new((*new_type.expr_c).try_into()?),
                },
            },
            grammar_trait::Expr::Card(ExprCard { card }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::Card {
                    id: CardId((*card.uuid).try_into()?),
                    item: Box::new((*card.expr_c).try_into()?),
                    next: Box::new((*card.expr_c0).try_into()?),
                },
            },
            grammar_trait::Expr::Brand(ExprBrand { brand }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: Expr::DeclareBrand {
                    brand: (*brand.ident).into(),
                    item: Box::new((*brand.expr_c).try_into()?),
                },
            },
        };
        Ok(expr)
    }
}

impl TryFrom<grammar_trait::Ty<'_>> for WithMeta<ast::ty::Type> {
    type Error = MinimalistSyntaxError;
    fn try_from(ty: grammar_trait::Ty) -> Result<Self, Self::Error> {
        let comments = vec![].into();
        let ty = match ty {
            grammar_trait::Ty::ExprBeginTyExprEnd(TyExprBeginTyExprEnd { ty }) => {
                (*ty).try_into()?
            }
            grammar_trait::Ty::Infer(TyInfer { infer: _ }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Infer,
            },
            grammar_trait::Ty::RealKey(TyRealKey { .. }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Real,
            },
            grammar_trait::Ty::RationalKey(TyRationalKey { .. }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Rational,
            },
            grammar_trait::Ty::IntegerKey(TyIntegerKey { .. }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Integer,
            },
            grammar_trait::Ty::StringKey(TyStringKey { string_key: _ }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::String,
            },
            grammar_trait::Ty::Effectful(TyEffectful { effectful }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Effectful {
                    ty: Box::new((*effectful.ty).try_into()?),
                    effects: (*effectful.effect_expr).try_into()?,
                },
            },
            grammar_trait::Ty::ProductTy(TyProductTy { product_ty }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Product(
                    product_ty
                        .product_ty_list
                        .into_iter()
                        .map(|t| (*t.ty).try_into())
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                ),
            },
            grammar_trait::Ty::Sum(TySum { sum }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Sum(
                    sum.sum_list
                        .into_iter()
                        .map(|t| (*t.ty).try_into())
                        .collect::<Result<_, MinimalistSyntaxError>>()?,
                ),
            },
            grammar_trait::Ty::VecTy(TyVecTy { vec_ty }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Vector(Box::new((*vec_ty.ty).try_into()?)),
            },
            grammar_trait::Ty::MapTy(TyMapTy { map_ty }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Map {
                    key: Box::new((*map_ty.ty).try_into()?),
                    value: Box::new((*map_ty.ty0).try_into()?),
                },
            },
            grammar_trait::Ty::FunctionTy(TyFunctionTy { function_ty }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Function(Box::new(Function {
                    parameter: (*function_ty.ty).try_into()?,
                    body: (*function_ty.ty0).try_into()?,
                })),
            },
            grammar_trait::Ty::LabeledTy(TyLabeledTy { labeled_ty }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Labeled {
                    brand: (*labeled_ty.label.ident).into(),
                    item: Box::new((*labeled_ty.ty).try_into()?),
                },
            },
            grammar_trait::Ty::AttributedTy(TyAttributedTy { attributed_ty }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Attributed {
                    attr: (*attributed_ty.attribute.expr_c).try_into()?,
                    ty: Box::new((*attributed_ty.ty).try_into()?),
                },
            },
            grammar_trait::Ty::Variable(TyVariable { variable }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Variable((*variable.ident).into()),
            },
            grammar_trait::Ty::LetTy(TyLetTy { let_ty }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::Type::Let {
                    variable: (*let_ty.ident).into(),
                    definition: Box::new((*let_ty.ty).try_into()?),
                    body: Box::new((*let_ty.ty0).try_into()?),
                },
            },
            grammar_trait::Ty::ForallTy(TyForallTy { forall_ty }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
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
            grammar_trait::Ty::ExistsTy(TyExistsTy { exists_ty }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
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

impl TryFrom<grammar_trait::EffectExpr<'_>> for WithMeta<ast::ty::EffectExpr> {
    type Error = MinimalistSyntaxError;
    fn try_from(effect_expr: grammar_trait::EffectExpr) -> Result<Self, Self::Error> {
        let comments = vec![].into();
        let effect_expr = match effect_expr {
            grammar_trait::EffectExpr::Effects(EffectExprEffects { effects }) => WithMeta {
                meta: Meta {
                    id: NodeId::new(),
                    comments,
                },
                value: ast::ty::EffectExpr::Effects(
                    effects
                        .effects_list
                        .into_iter()
                        .map(|effect| (*effect.effect).try_into())
                        .collect::<Result<Vec<_>, _>>()?,
                ),
            },
            grammar_trait::EffectExpr::AddEffects(EffectExprAddEffects { add_effects }) => {
                WithMeta {
                    meta: Meta {
                        id: NodeId::new(),
                        comments,
                    },
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
                WithMeta {
                    meta: Meta {
                        id: NodeId::new(),
                        comments,
                    },
                    value: ast::ty::EffectExpr::Sub {
                        minuend: Box::new((*sub_effects.effect_expr).try_into()?),
                        subtrahend: Box::new((*sub_effects.effect_expr0).try_into()?),
                    },
                }
            }
            grammar_trait::EffectExpr::ApplyEffects(EffectExprApplyEffects { apply_effects }) => {
                WithMeta {
                    meta: Meta {
                        id: NodeId::new(),
                        comments,
                    },
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

impl TryFrom<grammar_trait::Effect<'_>> for WithMeta<ast::ty::Effect> {
    type Error = MinimalistSyntaxError;
    fn try_from(effect: grammar_trait::Effect) -> Result<Self, Self::Error> {
        let comments = vec![].into();
        Ok(WithMeta {
            meta: Meta {
                id: NodeId::new(),
                comments,
            },
            value: ast::ty::Effect {
                input: (*effect.ty).try_into()?,
                output: (*effect.ty0).try_into()?,
            },
        })
    }
}

impl TryFrom<grammar_trait::ExprC<'_>> for Dson {
    type Error = MinimalistSyntaxError;

    fn try_from(value: grammar_trait::ExprC<'_>) -> Result<Self, Self::Error> {
        let expr: WithMeta<Expr> = value.try_into()?;
        expr.try_into().map_err(MinimalistSyntaxError::DsonError)
    }
}

impl TryFrom<grammar_trait::Uuid<'_>> for Uuid {
    type Error = MinimalistSyntaxError;

    fn try_from(value: grammar_trait::Uuid<'_>) -> Result<Self, Self::Error> {
        let value = value.uuid_text.uuid_text.text().to_string();
        Uuid::from_str(&value).map_err(MinimalistSyntaxError::UuidError)
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

impl From<grammar_trait::Comment<'_>> for ast::meta::Comment {
    fn from(comment: grammar_trait::Comment) -> Self {
        match comment {
            grammar_trait::Comment::BlockComment(CommentBlockComment { block_comment }) => {
                ast::meta::Comment::Block(
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
                            .collect::<String>(),
                )
            }
            grammar_trait::Comment::InlineComment(CommentInlineComment { inline_comment }) => {
                ast::meta::Comment::Line(
                    inline_comment
                        .comment_characters
                        .comment_characters
                        .text()
                        .to_string(),
                )
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
