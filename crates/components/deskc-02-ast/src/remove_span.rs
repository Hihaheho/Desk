use crate::{
    expr::Expr,
    span::WithSpan,
    ty::{EffectExpr, Type},
};

pub fn remove_span(WithSpan { id, span, value }: &mut WithSpan<Expr>) {
    match value {
        Expr::Literal(_) => {}
        Expr::Let {
            ty,
            definition,
            body,
        } => {
            remove_span_ty(ty);
            remove_span(definition);
            remove_span(body);
        }
        Expr::Perform { input, output } => {
            remove_span(input);
            remove_span_ty(output);
        }
        Expr::Continue { input, output } => {
            remove_span(input);
            if let Some(output) = output {
                remove_span_ty(output);
            }
        }
        Expr::Handle { expr, handlers } => {
            remove_span(expr);
            for handler in handlers {
                remove_span_ty(&mut handler.input);
                remove_span_ty(&mut handler.output);
                remove_span(&mut handler.handler);
            }
        }
        Expr::Apply {
            function,
            link_name: _,
            arguments,
        } => {
            remove_span_ty(function);
            for arg in arguments {
                remove_span(arg);
            }
        }
        Expr::Product(exprs) => {
            for expr in exprs {
                remove_span(expr);
            }
        }
        Expr::Match { of, cases } => {
            remove_span(of);
            for case in cases {
                remove_span_ty(&mut case.ty);
                remove_span(&mut case.expr);
            }
        }
        Expr::Typed { ty, item } => {
            remove_span_ty(ty);
            remove_span(item);
        }
        Expr::Hole => {}
        Expr::Function { parameters, body } => {
            for param in parameters {
                remove_span_ty(param);
            }
            remove_span(body);
        }
        Expr::Vector(exprs) => {
            for expr in exprs {
                remove_span(expr);
            }
        }
        Expr::Set(exprs) => {
            for expr in exprs {
                remove_span(expr);
            }
        }
        Expr::Import { ty, .. } => {
            remove_span_ty(ty);
        }
        Expr::Export { ty } => {
            remove_span_ty(ty);
        }
        Expr::Attribute { attr, item } => {
            remove_span(attr);
            remove_span(item);
        }
        Expr::Brand { brands: _, item } => {
            remove_span(item);
        }
        Expr::Label { label: _, item } => {
            remove_span(item);
        }
        Expr::NewType { ident: _, ty, expr } => {
            remove_span_ty(ty);
            remove_span(expr);
        }
        Expr::Comment {
            position: _,
            text: _,
            item,
        } => {
            remove_span(item);
        }
        Expr::Card {
            uuid: _,
            item,
            next,
        } => {
            remove_span(item);
            if let Some(next) = next {
                remove_span(next);
            }
        }
    }
    *id = Default::default();
    *span = Default::default();
}

pub fn remove_span_ty(WithSpan { id, span, value }: &mut WithSpan<Type>) {
    match value {
        Type::Brand { brand: _, item } => {
            remove_span_ty(item);
        }
        Type::Number => {}
        Type::String => {}
        Type::Trait(types) => {
            for ty in types {
                remove_span_ty(ty);
            }
        }
        Type::Effectful { ty, effects } => {
            remove_span_ty(ty);
            remove_span_effects(effects);
        }
        Type::Infer => {}
        Type::This => {}
        Type::Product(types) => {
            for ty in types {
                remove_span_ty(ty);
            }
        }
        Type::Sum(types) => {
            for ty in types {
                remove_span_ty(ty);
            }
        }
        Type::Function { parameters, body } => {
            for param in parameters {
                remove_span_ty(param);
            }
            remove_span_ty(body);
        }
        Type::Vector(ty) => {
            remove_span_ty(ty);
        }
        Type::Set(ty) => {
            remove_span_ty(ty);
        }
        Type::Let { variable: _, body } => {
            remove_span_ty(body);
        }
        Type::Variable(_) => {}
        Type::BoundedVariable {
            bound,
            identifier: _,
        } => {
            remove_span_ty(bound);
        }
        Type::Attribute { attr, ty } => {
            remove_span(attr);
            remove_span_ty(ty);
        }
        Type::Comment {
            position: _,
            text: _,
            item,
        } => {
            remove_span_ty(item);
        }
    }
    *id = Default::default();
    *span = Default::default();
}

pub fn remove_span_effects(WithSpan { id, span, value }: &mut WithSpan<EffectExpr>) {
    match value {
        EffectExpr::Effects(effects) => {
            for effect in effects {
                effect.id = Default::default();
                effect.span = Default::default();
                remove_span_ty(&mut effect.value.input);
                remove_span_ty(&mut effect.value.output);
            }
        }
        EffectExpr::Add(exprs) => {
            for expr in exprs {
                remove_span_effects(expr);
            }
        }
        EffectExpr::Sub {
            minuend,
            subtrahend,
        } => {
            remove_span_effects(minuend);
            remove_span_effects(subtrahend);
        }
        EffectExpr::Apply {
            function,
            arguments,
        } => {
            remove_span_ty(function);
            for arg in arguments {
                remove_span_ty(arg);
            }
        }
    }
    *id = Default::default();
    *span = Default::default();
}
