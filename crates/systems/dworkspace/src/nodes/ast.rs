use std::sync::Arc;

use components::{code::Code, content::Content, node::Node};
use deskc::parse_source_code;
use deskc_ast::{
    expr::{Expr, Literal},
    span::WithSpan,
    ty::{Effect, EffectExpr, Type},
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
        Content::Float(float) => Expr::Literal(Literal::Float(*float)),
        Content::Apply { ty, link_name } => Expr::Apply {
            function: from_types(ty),
            link_name: link_name.clone(),
            arguments: node
                .operands
                .iter()
                .map(|node| match genast(node)? {
                    Code::SourceCode { syntax, source } => Ok(parse_source_code(&syntax, &source)?),
                    Code::Ast(ast) => Ok(ast.as_ref().clone()),
                })
                .collect::<Result<Vec<_>, anyhow::Error>>()?,
        },
    };
    Ok(Code::Ast(Arc::new(WithSpan {
        id: node.id.clone(),
        value: expr,
        span: 0..0,
    })))
}

fn from_types(ty: &deskc_types::Type) -> WithSpan<Type> {
    use deskc_types::Type::*;
    let value = match ty {
        Number => Type::Number,
        String => Type::String,
        Product(types) => Type::Product(types.iter().map(from_types).collect()),
        Sum(types) => Type::Sum(types.iter().map(from_types).collect()),
        Function { parameters, body } => Type::Function {
            parameters: parameters.iter().map(from_types).collect(),
            body: Box::new(from_types(body)),
        },
        Vector(ty) => Type::Vector(Box::new(from_types(ty))),
        Set(ty) => Type::Set(Box::new(from_types(ty))),
        Variable(ident) => Type::Variable(ident.clone()),
        ForAll { .. } => todo!(),
        Effectful { ty, effects } => Type::Effectful {
            ty: Box::new(from_types(ty)),
            effects: from_types_effects(effects),
        },
        Brand { brand, item } => Type::Brand {
            brand: brand.clone(),
            item: Box::new(from_types(item)),
        },
        Label { label, item } => Type::Brand {
            brand: label.clone(),
            item: Box::new(from_types(item)),
        },
    };
    WithSpan {
        id: NodeId::new(),
        span: 0..0,
        value,
    }
}

fn from_types_effects(effects: &deskc_types::EffectExpr) -> WithSpan<EffectExpr> {
    let value = match effects {
        deskc_types::EffectExpr::Effects(effects) => EffectExpr::Effects(
            effects
                .iter()
                .map(|deskc_types::Effect { input, output }| WithSpan {
                    id: NodeId::new(),
                    span: 0..0,
                    value: Effect {
                        input: from_types(input),
                        output: from_types(output),
                    },
                })
                .collect(),
        ),
        deskc_types::EffectExpr::Add(exprs) => {
            EffectExpr::Add(exprs.iter().map(from_types_effects).collect())
        }
        deskc_types::EffectExpr::Sub {
            minuend,
            subtrahend,
        } => EffectExpr::Sub {
            minuend: Box::new(from_types_effects(minuend)),
            subtrahend: Box::new(from_types_effects(subtrahend)),
        },
        deskc_types::EffectExpr::Apply {
            function,
            arguments,
        } => EffectExpr::Apply {
            function: Box::new(from_types(function)),
            arguments: arguments.iter().map(from_types).collect(),
        },
    };
    WithSpan {
        id: NodeId::new(),
        span: 0..0,
        value,
    }
}
