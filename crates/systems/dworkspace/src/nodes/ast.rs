use std::sync::Arc;

use components::{code::Code, content::Content, node::Node};
use deskc::parse_source_code;
use deskc_ast::{
    expr::{Expr, Literal},
    meta::{Meta, WithMeta},
    ty::{Effect, EffectExpr, Function, Type},
};
use deskc_ids::NodeId;

use crate::query_error::QueryError;

use super::NodeQueries;

pub(super) fn ast(db: &dyn NodeQueries, node_id: NodeId) -> Result<Code, QueryError> {
    let ast = db.node(node_id);

    Ok(genast(&ast)?)
}

fn genast(node: &Node) -> Result<Code, anyhow::Error> {
    let expr = match &node.content {
        Content::SourceCode { syntax, source } => {
            return Ok(Code::SourceCode {
                syntax: syntax.clone(),
                source: Arc::new(source.clone()),
            })
        }
        Content::String(string) => Expr::Literal(Literal::String(string.clone())),
        Content::Integer(integer) => Expr::Literal(Literal::Integer(*integer)),
        Content::Rational(a, b) => Expr::Literal(Literal::Rational(*a, *b)),
        Content::Real(float) => Expr::Literal(Literal::Real(*float)),
        Content::Apply { link_name } => Expr::Apply {
            function: todo!(),
            link_name: link_name.clone(),
            arguments: node
                .operands
                .iter()
                .map(|node| match genast(node)? {
                    Code::SourceCode { syntax, source } => {
                        Ok(parse_source_code(&syntax, &source)?.expr.as_ref().clone())
                    }
                    Code::Ast(ast) => Ok(ast.as_ref().clone()),
                })
                .collect::<Result<Vec<_>, anyhow::Error>>()?,
        },
        _ => todo!(),
    };
    Ok(Code::Ast(Arc::new(WithMeta {
        meta: node.id.clone().into(),
        value: expr,
    })))
}

fn from_types(ty: &deskc_ty::Type) -> WithMeta<Type> {
    use deskc_ty::Type as DeskcType;
    let value = match ty {
        DeskcType::Real => Type::Real,
        DeskcType::Rational => Type::Rational,
        DeskcType::Integer => Type::Integer,
        DeskcType::String => Type::String,
        DeskcType::Product(types) => Type::Product(types.iter().map(from_types).collect()),
        DeskcType::Sum(types) => Type::Sum(types.iter().map(from_types).collect()),
        DeskcType::Function(function) => Type::Function(Box::new(Function {
            parameter: from_types(&function.parameter),
            body: from_types(&function.body),
        })),
        DeskcType::Vector(ty) => Type::Vector(Box::new(from_types(ty))),
        DeskcType::Map { key, value } => Type::Map {
            key: Box::new(from_types(key)),
            value: Box::new(from_types(value)),
        },
        DeskcType::Variable(ident) => Type::Variable(ident.clone()),
        DeskcType::ForAll { .. } => todo!(),
        DeskcType::Effectful { ty, effects } => Type::Effectful {
            ty: Box::new(from_types(ty)),
            effects: from_types_effects(effects),
        },
        DeskcType::Brand { brand, item } => Type::Labeled {
            brand: brand.clone(),
            item: Box::new(from_types(item)),
        },
        DeskcType::Label { label, item } => Type::Labeled {
            brand: label.clone(),
            item: Box::new(from_types(item)),
        },
    };
    WithMeta {
        meta: Meta::new_no_comments(),
        value,
    }
}

fn from_types_effects(effects: &deskc_ty::EffectExpr) -> WithMeta<EffectExpr> {
    let value = match effects {
        deskc_ty::EffectExpr::Effects(effects) => EffectExpr::Effects(
            effects
                .iter()
                .map(|deskc_ty::Effect { input, output }| WithMeta {
                    meta: Meta::new_no_comments(),
                    value: Effect {
                        input: from_types(input),
                        output: from_types(output),
                    },
                })
                .collect(),
        ),
        deskc_ty::EffectExpr::Add(exprs) => {
            EffectExpr::Add(exprs.iter().map(from_types_effects).collect())
        }
        deskc_ty::EffectExpr::Sub {
            minuend,
            subtrahend,
        } => EffectExpr::Sub {
            minuend: Box::new(from_types_effects(minuend)),
            subtrahend: Box::new(from_types_effects(subtrahend)),
        },
        deskc_ty::EffectExpr::Apply {
            function,
            arguments,
        } => EffectExpr::Apply {
            function: Box::new(from_types(function)),
            arguments: arguments.iter().map(from_types).collect(),
        },
    };
    WithMeta {
        meta: Meta::new_no_comments(),
        value,
    }
}
