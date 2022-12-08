use crate::{
    expr::Expr,
    span::WithSpan,
    ty::{EffectExpr, Type},
};

pub fn remove_span(WithSpan { id, span, value }: &mut WithSpan<Expr>) {
    match value {
        Expr::Literal(_) => {}
        Expr::Do { stmt, expr } => {
            remove_span(stmt);
            remove_span(expr);
        }
        Expr::Let { definition, body } => {
            remove_span(definition);
            remove_span(body);
        }
        Expr::Perform { input, output } => {
            remove_span(input);
            remove_span_ty(output);
        }
        Expr::Continue { input, output } => {
            remove_span(input);
            remove_span_ty(output);
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
        Expr::Function { parameter, body } => {
            remove_span_ty(parameter);
            remove_span(body);
        }
        Expr::Vector(exprs) => {
            for expr in exprs {
                remove_span(expr);
            }
        }
        Expr::Map(exprs) => {
            for elem in exprs {
                remove_span(&mut elem.key);
                remove_span(&mut elem.value);
            }
        }
        Expr::Attributed { attr: _, item } => {
            remove_span(item);
        }
        Expr::Brand { brand: _, item } => {
            remove_span(item);
        }
        Expr::Label { label: _, item } => {
            remove_span(item);
        }
        Expr::NewType { ident: _, ty, expr } => {
            remove_span_ty(ty);
            remove_span(expr);
        }
        Expr::Comment { text: _, item } => {
            remove_span(item);
        }
        Expr::Card { id: _, item, next } => {
            remove_span(item);
            remove_span(next);
        }
    }
    *id = Default::default();
    *span = Default::default();
}

pub fn remove_span_ty(WithSpan { id, span, value }: &mut WithSpan<Type>) {
    match value {
        Type::Labeled { brand: _, item } => {
            remove_span_ty(item);
        }
        Type::Real => {}
        Type::Rational => {}
        Type::Integer => {}
        Type::String => {}
        Type::Trait(trait_) => {
            for ty in trait_ {
                ty.id = Default::default();
                ty.span = Default::default();
                remove_span_ty(&mut ty.value.parameter);
                remove_span_ty(&mut ty.value.body);
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
        Type::Function(function) => {
            remove_span_ty(&mut function.parameter);
            remove_span_ty(&mut function.body);
        }
        Type::Vector(ty) => {
            remove_span_ty(ty);
        }
        Type::Map { key, value } => {
            remove_span_ty(key);
            remove_span_ty(value);
        }
        Type::Let {
            variable: _,
            definition,
            body,
        } => {
            remove_span_ty(definition);
            remove_span_ty(body);
        }
        Type::Variable(_) => {}
        Type::Attributed { attr: _, ty } => {
            remove_span_ty(ty);
        }
        Type::Comment { text: _, item } => {
            remove_span_ty(item);
        }
        Type::Forall {
            variable: _,
            bound,
            body,
        } => {
            if let Some(bound) = bound {
                remove_span_ty(bound);
            }
            remove_span_ty(body);
        }
        Type::Exists {
            variable: _,
            bound,
            body,
        } => {
            if let Some(bound) = bound {
                remove_span_ty(bound);
            }
            remove_span_ty(body);
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
