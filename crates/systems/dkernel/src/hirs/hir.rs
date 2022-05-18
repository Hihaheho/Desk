use std::sync::Arc;

use components::{content::Content, node::Node};
use deskc_hir::{
    expr::{Expr, Literal},
    meta::{Meta, WithMeta},
    ty::{Effect, EffectExpr, Type},
};
use deskc_hirgen::gen_hir;
use deskc_ids::NodeId;
use deskc_lexer::scan;
use deskc_parser::parse;

use crate::{
    error::KernelError,
    query_result::{QueryError, QueryResult},
};

use super::HirQueries;

pub(super) fn hir(db: &dyn HirQueries, node_id: NodeId) -> QueryResult<WithMeta<Expr>> {
    let ast = db.ast(node_id);

    Ok(Arc::new(genhir(&ast)?))
}

fn genhir(node: &Node) -> Result<WithMeta<Expr>, QueryError> {
    let meta = Meta {
        attrs: vec![],
        file_id: node.file_id.clone(),
        span: None,
    };
    let expr = match &node.content {
        Content::Source(source) => {
            let tokens = scan(source)?;
            let ast = parse(tokens)?;
            let (_, hir) = gen_hir(&ast)?;
            return Ok(hir.entrypoint.ok_or(KernelError::NoEntrypoint {
                node_id: node.id.clone(),
            })?);
        }
        Content::String(string) => Expr::Literal(Literal::String(string.clone())),
        Content::Integer(integer) => Expr::Literal(Literal::Integer(*integer)),
        Content::Rational(a, b) => Expr::Literal(Literal::Rational(*a, *b)),
        Content::Float(float) => Expr::Literal(Literal::Float(*float)),
        Content::Apply { ty, link_name } => Expr::Apply {
            function: from_types(ty, meta.clone()),
            link_name: link_name.clone(),
            arguments: node
                .children
                .iter()
                .map(genhir)
                .collect::<Result<Vec<_>, _>>()?,
        },
    };
    Ok(WithMeta {
        id: node.id.clone(),
        meta,
        value: expr,
    })
}

fn from_types(ty: &deskc_types::Type, meta: Meta) -> WithMeta<Type> {
    use deskc_types::Type::*;
    let value = match ty {
        Number => Type::Number,
        String => Type::String,
        Product(types) => {
            Type::Product(types.iter().map(|t| from_types(t, meta.clone())).collect())
        }
        Sum(types) => Type::Sum(types.iter().map(|t| from_types(t, meta.clone())).collect()),
        Function { parameters, body } => Type::Function {
            parameters: parameters
                .iter()
                .map(|t| from_types(t, meta.clone()))
                .collect(),
            body: Box::new(from_types(body, meta.clone())),
        },
        Vector(ty) => Type::Vector(Box::new(from_types(ty, meta.clone()))),
        Set(ty) => Type::Set(Box::new(from_types(ty, meta.clone()))),
        Variable(ident) => Type::Variable(*ident),
        ForAll { .. } => todo!(),
        Effectful { ty, effects } => Type::Effectful {
            ty: Box::new(from_types(ty, meta.clone())),
            effects: from_types_effects(effects, meta.clone()),
        },
        Brand { brand, item } => Type::Brand {
            brand: brand.clone(),
            item: Box::new(from_types(item, meta.clone())),
        },
        Label { label, item } => Type::Label {
            label: label.clone(),
            item: Box::new(from_types(item, meta.clone())),
        },
    };
    WithMeta {
        id: NodeId::new(),
        meta,
        value,
    }
}

fn from_types_effects(effects: &deskc_types::EffectExpr, meta: Meta) -> WithMeta<EffectExpr> {
    let value = match effects {
        deskc_types::EffectExpr::Effects(effects) => EffectExpr::Effects(
            effects
                .iter()
                .map(|deskc_types::Effect { input, output }| WithMeta {
                    id: NodeId::new(),
                    meta: meta.clone(),
                    value: Effect {
                        input: from_types(input, meta.clone()),
                        output: from_types(output, meta.clone()),
                    },
                })
                .collect(),
        ),
        deskc_types::EffectExpr::Add(exprs) => EffectExpr::Add(
            exprs
                .iter()
                .map(|expr| from_types_effects(expr, meta.clone()))
                .collect(),
        ),
        deskc_types::EffectExpr::Sub {
            minuend,
            subtrahend,
        } => EffectExpr::Sub {
            minuend: Box::new(from_types_effects(minuend, meta.clone())),
            subtrahend: Box::new(from_types_effects(subtrahend, meta.clone())),
        },
        deskc_types::EffectExpr::Apply {
            function,
            arguments,
        } => EffectExpr::Apply {
            function: Box::new(from_types(function, meta.clone())),
            arguments: arguments
                .iter()
                .map(|expr| from_types(expr, meta.clone()))
                .collect(),
        },
    };
    WithMeta {
        id: NodeId::new(),
        meta,
        value,
    }
}
