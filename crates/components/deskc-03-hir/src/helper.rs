use crate::{
    expr::{Expr, Handler, MatchCase},
    meta::{dummy_meta, WithMeta},
    ty::Type,
};

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
        Type::Product(types) => Type::Product(types.into_iter().map(remove_meta_ty).collect()),
        Type::Sum(types) => Type::Sum(types.into_iter().map(remove_meta_ty).collect()),
        Type::Function { parameters, body } => Type::Function {
            parameters: parameters.into_iter().map(remove_meta_ty).collect(),
            body: Box::new(remove_meta_ty(*body)),
        },
        Type::Vector(ty) => Type::Vector(Box::new(remove_meta_ty(*ty))),
        Type::Set(ty) => Type::Set(Box::new(remove_meta_ty(*ty))),
        Type::Let { variable, body } => Type::Let {
            variable,
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
    dummy_meta(value)
}
pub fn remove_meta(expr: WithMeta<Expr>) -> WithMeta<Expr> {
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
        Expr::Continue { input, output } => Expr::Continue {
            input: Box::new(remove_meta(*input)),
            output: output.map(remove_meta_ty),
        },
        Expr::Handle { handlers, expr } => Expr::Handle {
            handlers: handlers
                .into_iter()
                .map(
                    |Handler {
                         input,
                         output,
                         handler,
                     }| Handler {
                        input: remove_meta_ty(input),
                        output: remove_meta_ty(output),
                        handler: remove_meta(handler),
                    },
                )
                .collect(),
            expr: Box::new(remove_meta(*expr)),
        },
        Expr::Apply {
            function,
            link_name,
            arguments,
        } => Expr::Apply {
            function: remove_meta_ty(function),
            link_name,
            arguments: arguments.into_iter().map(remove_meta).collect(),
        },
        Expr::Product(exprs) => Expr::Product(exprs.into_iter().map(remove_meta).collect()),
        Expr::Typed { ty, item: expr } => Expr::Typed {
            ty: remove_meta_ty(ty),
            item: Box::new(remove_meta(*expr)),
        },
        Expr::Function { parameter, body } => Expr::Function {
            parameter: remove_meta_ty(parameter),
            body: Box::new(remove_meta(*body)),
        },
        Expr::Vector(exprs) => Expr::Vector(exprs.into_iter().map(remove_meta).collect()),
        Expr::Set(exprs) => Expr::Set(exprs.into_iter().map(remove_meta).collect()),
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
    let mut with_meta = dummy_meta(value);
    with_meta.meta.attrs = expr.meta.attrs;
    with_meta
}
