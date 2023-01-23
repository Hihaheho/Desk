use ast::{
    expr::{Expr, LinkName},
    meta::{Comment, Meta, WithMeta},
    parser::Parser,
    ty::{Effect, EffectExpr, Function, Type},
};
use dson::{Dson, MapElem};
use proc_macro::TokenStream;
use quote::quote;
use uuid::Uuid;

#[proc_macro]
pub fn ty(item: TokenStream) -> TokenStream {
    fn input(string: String) -> String {
        "&".to_string() + &string
    }
    fn map(expr: &WithMeta<Expr>) -> proc_macro2::TokenStream {
        if let Expr::Apply { function, .. } = &expr.value {
            from_type(&function.value)
        } else {
            panic!("Failed to parse reference")
        }
    }
    parse(
        item,
        input,
        map,
        quote! {
            use deskc_type::{Effect, Function, Type, EffectExpr};
            use dson::{Dson, Literal};
        },
    )
}

#[proc_macro]
pub fn effect(item: TokenStream) -> TokenStream {
    fn input(string: String) -> String {
        format!("& ! {{ {string} }} 'integer")
    }
    fn map(expr: &WithMeta<Expr>) -> proc_macro2::TokenStream {
        let Expr::Apply { function, .. } = &expr.value
         else {
            panic!("Failed to parse reference")
        };
        let Type::Effectful { ty: _, effects } = &function.value else {
            panic!("Failed to parse effectful")
        };
        let EffectExpr::Effects(effects) = &effects.value else {
            panic!("Failed to parse effects")
        };
        from_effect(&effects[0].value)
    }
    parse(
        item,
        input,
        map,
        quote! {
            use deskc_type::{Effect, Function, Type, EffectExpr};
            use dson::{Dson, Literal};
        },
    )
}

#[proc_macro]
pub fn dson(item: TokenStream) -> TokenStream {
    fn input(string: String) -> String {
        format!("# {string} 1")
    }
    fn map(expr: &WithMeta<Expr>) -> proc_macro2::TokenStream {
        let Expr::Attributed { attr, item: _ } = &expr.value else {
            panic!("Failed to parse label")
        };
        from_dson(attr)
    }
    parse(
        item,
        input,
        map,
        quote! {
            use dson::{Dson, Literal};
        },
    )
}

#[proc_macro]
pub fn ast(item: TokenStream) -> TokenStream {
    fn input(string: String) -> String {
        string
    }
    fn map(expr: &WithMeta<Expr>) -> proc_macro2::TokenStream {
        from_expr(expr)
    }
    parse(
        item,
        input,
        map,
        quote! {
            use deskc_ast::{
                expr::{Expr, Literal, MatchCase},
                meta::{WithMeta, Meta, Comment, Comments},
                ty::{Effect, EffectExpr, Function, Type},
            };
            use deskc_ids::LinkName;
            use dson::Dson;
        },
    )
}

fn parse(
    item: TokenStream,
    input: fn(String) -> String,
    map: fn(&WithMeta<Expr>) -> proc_macro2::TokenStream,
    uses: proc_macro2::TokenStream,
) -> TokenStream {
    if let Some(literal) = item.into_iter().next() {
        let string = literal.to_string();
        let string = if string.starts_with('"') {
            // Remove the quotes
            string[1..string.len() - 1].to_string()
        } else if string.starts_with("r#") {
            // Remove the delimiters
            string[3..string.len() - 2].to_string()
        } else {
            return quote! {
                compile_error!("The first argument must be a string literal")
            }
            .into();
        };
        match parser::MinimalistSyntaxParser::parse(&input(string)) {
            Ok(expr) => {
                let tokens = map(&expr.expr);
                quote! {
                    {
                        #uses
                        #tokens
                    }
                }
                .into()
            }
            Err(err) => {
                let err = format!("{err:?}");
                quote! {
                    compile_error!(#err)
                }
                .into()
            }
        }
    } else {
        quote! {
            compile_error!("The first argument must be a string literal")
        }
        .into()
    }
}

fn from_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Labeled { brand, item } => {
            let item = from_type(&item.value);
            quote! {
                Type::Label {
                    label: #brand.to_string(),
                    item: Box::new(#item),
                }
            }
        }
        Type::Real => quote! { Type::Real },
        Type::Rational => quote! { Type::Rational },
        Type::Integer => quote! { Type::Integer },
        Type::String => quote! { Type::String },
        Type::Effectful { ty, effects } => {
            let ty = from_type(&ty.value);
            let effects = from_effect_expr(&effects.value);
            quote! {
                Type::Effectful {
                    ty: Box::new(#ty),
                    effects: #effects,
                }
            }
        }
        Type::Infer => quote! { Type::Infer },
        Type::Product(types) => {
            let types = types
                .into_iter()
                .map(|ty| from_type(&ty.value))
                .collect::<Vec<_>>();
            quote! {
                Type::Product(vec![#(#types),*])
            }
        }
        Type::Sum(types) => {
            let types = types
                .into_iter()
                .map(|ty| from_type(&ty.value))
                .collect::<Vec<_>>();
            quote! {
                Type::Sum(vec![#(#types),*])
            }
        }
        Type::Function(function) => {
            let Function { parameter, body } = &**function;
            let parameter = from_type(&parameter.value);
            let body = from_type(&body.value);
            quote! {
                Type::Function(Box::new(Function {
                    parameter: #parameter,
                    body: #body,
                }))
            }
        }
        Type::Vector(ty) => {
            let ty = from_type(&ty.value);
            quote! {
                Type::Vector(Box::new(#ty))
            }
        }
        Type::Map { key, value } => {
            let key = from_type(&key.value);
            let value = from_type(&value.value);
            quote! {
                Type::Map {
                    key: Box::new(#key),
                    value: Box::new(#value),
                }
            }
        }
        Type::Let {
            variable,
            definition,
            body,
        } => {
            let definition = from_type(&definition.value);
            let body = from_type(&body.value);
            quote! {
                Type::Let {
                    variable: #variable.into(),
                    definition: Box::new(#definition),
                    body: Box::new(#body),
                }
            }
        }
        Type::Variable(ident) => quote! { Type::Variable(#ident.into()) },
        Type::Attributed { attr, ty } => {
            let attr = from_dson(&attr);
            let ty = from_type(&ty.value);
            quote! {
                Type::Attributed {
                    attr: #attr,
                    ty: Box::new(#ty),
                }
            }
        }
        Type::Forall {
            variable,
            bound,
            body,
        } => {
            let bound = if let Some(bound) = bound {
                let bound = from_type(&bound.value);
                quote! { Some(Box::new(#bound)) }
            } else {
                quote! { None }
            };
            let body = from_type(&body.value);
            quote! {
                Type::Forall {
                    variable: #variable.into(),
                    bound: Box::new(#bound),
                    body: Box::new(#body),
                }
            }
        }
        Type::Exists {
            variable,
            bound,
            body,
        } => {
            let bound = if let Some(bound) = bound {
                let bound = from_type(&bound.value);
                quote! { Some(Box::new(#bound)) }
            } else {
                quote! { None }
            };
            let body = from_type(&body.value);
            quote! {
                Type::Exists {
                    variable: #variable.into(),
                    bound: Box::new(#bound),
                    body: Box::new(#body),
                }
            }
        }
    }
}

fn from_dson(dson: &Dson) -> proc_macro2::TokenStream {
    match dson {
        Dson::Literal(literal) => match literal {
            dson::Literal::String(string) => {
                quote! { Dson::Literal(dson::Literal::String(#string.into())) }
            }
            dson::Literal::Integer(integer) => {
                quote! { Dson::Literal(dson::Literal::Integer(#integer)) }
            }
            dson::Literal::Rational(a, b) => {
                quote! { Dson::Literal(dson::Literal::Rational(#a, #b)) }
            }
            dson::Literal::Real(real) => {
                let real = real.0;
                quote! { Dson::Literal(dson::Literal::Real(#real)) }
            }
        },
        Dson::Product(dsons) => {
            let dsons = dsons.into_iter().map(from_dson).collect::<Vec<_>>();
            quote! {
                Dson::Product(vec![#(#dsons),*])
            }
        }
        Dson::Vector(dsons) => {
            let dsons = dsons.into_iter().map(from_dson).collect::<Vec<_>>();
            quote! {
                Dson::Vector(vec![#(#dsons),*])
            }
        }
        Dson::Map(elems) => {
            let elems = elems
                .into_iter()
                .map(|MapElem { key, value }| {
                    let key = from_dson(&key);
                    let value = from_dson(&value);
                    quote! { dson::MapElem { key: #key, value: #value } }
                })
                .collect::<Vec<_>>();
            quote! {
                Dson::Map(vec![#(#elems),*])
            }
        }
        Dson::Attributed { attr, expr } => {
            let attr = from_dson(&*attr);
            let expr = from_dson(&*expr);
            quote! {
                Dson::Attributed {
                    attr: Box::new(#attr),
                    expr: Box::new(#expr),
                }
            }
        }
        Dson::Labeled { label, expr } => {
            let expr = from_dson(&*expr);
            quote! {
                Dson::Labeled {
                    label: #label.to_string(),
                    expr: Box::new(#expr),
                }
            }
        }
        Dson::Typed { ty, expr } => {
            let ty = from_dson_type(&ty);
            let expr = from_dson(&*expr);
            quote! {
                Dson::Typed {
                    ty: Box::new(#ty),
                    expr: Box::new(#expr),
                }
            }
        }
        Dson::Comment { text, expr } => {
            let expr = from_dson(&*expr);
            quote! {
                Dson::Comment {
                    text: #text.into(),
                    expr: Box::new(#expr),
                }
            }
        }
    }
}

fn from_effect_expr(expr: &EffectExpr) -> proc_macro2::TokenStream {
    match expr {
        EffectExpr::Effects(effects) => {
            let effects = effects
                .into_iter()
                .map(|effect| from_effect(&effect.value))
                .collect::<Vec<_>>();
            quote! {
                EffectExpr::Effects(vec![#(#effects),*])
            }
        }
        EffectExpr::Add(exprs) => {
            let exprs = exprs
                .into_iter()
                .map(|expr| from_effect_expr(&expr.value))
                .collect::<Vec<_>>();
            quote! {
                EffectExpr::Add(vec![#(#exprs),*])
            }
        }
        EffectExpr::Sub {
            minuend,
            subtrahend,
        } => {
            let minuend = from_effect_expr(&minuend.value);
            let subtrahend = from_effect_expr(&subtrahend.value);
            quote! {
                EffectExpr::Sub {
                    minuend: Box::new(#minuend),
                    subtrahend: Box::new(#subtrahend),
                }
            }
        }
        EffectExpr::Apply {
            function,
            arguments,
        } => {
            let function = from_type(&function.value);
            let arguments = arguments
                .into_iter()
                .map(|argument| from_type(&argument.value))
                .collect::<Vec<_>>();
            quote! {
                EffectExpr::Apply {
                    function: Box::new(#function),
                    arguments: vec![#(#arguments),*],
                }
            }
        }
    }
}

fn from_effect(effect: &Effect) -> proc_macro2::TokenStream {
    let input = from_type(&effect.input.value);
    let output = from_type(&effect.output.value);
    quote! {
        Effect {
            input: #input,
            output: #output,
        }
    }
}

fn from_dson_type(ty: &dson::Type) -> proc_macro2::TokenStream {
    match ty {
        dson::Type::Brand { brand, item } => {
            let item = from_dson_type(&*item);
            quote! {
                dson::Type::Brand {
                    brand: #brand.to_string(),
                    item: Box::new(#item),
                }
            }
        }
        dson::Type::Real => quote! { dson::Type::Real },
        dson::Type::Rational => quote! { dson::Type::Rational },
        dson::Type::Integer => quote! { dson::Type::Integer },
        dson::Type::String => quote! { dson::Type::String },
        dson::Type::Product(types) => {
            let types = types.into_iter().map(from_dson_type).collect::<Vec<_>>();
            quote! {
                dson::Type::Product(vec![#(#types),*])
            }
        }
        dson::Type::Sum(types) => {
            let types = types.into_iter().map(from_dson_type).collect::<Vec<_>>();
            quote! {
                dson::Type::Sum(vec![#(#types),*])
            }
        }
        dson::Type::Vector(ty) => {
            let ty = from_dson_type(&*ty);
            quote! {
                dson::Type::Vector(Box::new(#ty))
            }
        }
        dson::Type::Map { key, value } => {
            let key = from_dson_type(&*key);
            let value = from_dson_type(&*value);
            quote! {
                dson::Type::Map {
                    key: Box::new(#key),
                    value: Box::new(#value),
                }
            }
        }
        dson::Type::Attributed { attr, ty } => {
            let attr = from_dson(&*attr);
            let ty = from_dson_type(&*ty);
            quote! {
                dson::Type::Attributed {
                    attr: Box::new(#attr),
                    ty: Box::new(#ty),
                }
            }
        }
        dson::Type::Comment { text, item } => {
            let item = from_dson_type(&*item);
            quote! {
                dson::Type::Comment {
                    text: #text.into(),
                    item: Box::new(#item),
                }
            }
        }
        dson::Type::Let {
            variable,
            definition,
            body,
        } => {
            let definition = from_dson_type(&*definition);
            let body = from_dson_type(&*body);
            quote! {
                dson::Type::Let {
                    variable: #variable.into(),
                    definition: Box::new(#definition),
                    body: Box::new(#body),
                }
            }
        }
        dson::Type::Variable(ident) => {
            quote! {
                dson::Type::Variable(#ident.into())
            }
        }
    }
}

fn from_expr(expr: &WithMeta<Expr>) -> proc_macro2::TokenStream {
    let tokens = match &expr.value {
        Expr::Literal(literal) => {
            let literal = match literal {
                ast::expr::Literal::String(string) => {
                    quote!(Literal::String(#string.into()))
                }
                ast::expr::Literal::Integer(integer) => {
                    quote!(Literal::Integer(#integer))
                }
                ast::expr::Literal::Rational(a, b) => {
                    quote!(Literal::Rational(#a, #b))
                }
                ast::expr::Literal::Real(real) => {
                    quote!(Literal::Real(#real))
                }
            };
            quote! {
                Expr::Literal(#literal)
            }
        }
        Expr::Do { stmt, expr } => {
            let stmt = from_expr(&*stmt);
            let expr = from_expr(&*expr);
            quote! {
                Expr::Do {
                    stmt: Box::new(#stmt),
                    expr: Box::new(#expr),
                }
            }
        }
        Expr::Let { definition, body } => {
            let definition = from_expr(&*definition);
            let body = from_expr(&*body);
            quote! {
                Expr::Let {
                    definition: Box::new(#definition),
                    body: Box::new(#body),
                }
            }
        }
        Expr::Perform { input, output } => {
            let input = from_expr(&*input);
            let output = from_type_for_ast(&output);
            quote! {
                Expr::Perform {
                    input: Box::new(#input),
                    output: #output,
                }
            }
        }
        Expr::Continue { input, output } => {
            let input = from_expr(&*input);
            let output = from_type_for_ast(&output);
            quote! {
                Expr::Continue {
                    input: Box::new(#input),
                    output: #output,
                }
            }
        }
        Expr::Handle { expr, handlers } => {
            let expr = from_expr(&*expr);
            let handlers = handlers
                .into_iter()
                .map(|handler| {
                    let expr = from_expr(&handler.value.handler);
                    let effect = from_effect_for_ast(&handler.value.effect);
                    let tokens = quote! {
                        Handler {
                            effect: #effect,
                            handler: #expr,
                        }
                    };
                    with_meta(&handler.meta, tokens)
                })
                .collect::<Vec<_>>();
            quote! {
                Expr::Handle {
                    expr: Box::new(#expr),
                    handlers: vec![#(#handlers),*],
                }
            }
        }
        Expr::Apply {
            function,
            link_name,
            arguments,
        } => {
            let function = from_type_for_ast(&function);
            let link_name = match link_name {
                LinkName::None => quote!(LinkName::None),
                LinkName::Version(vession) => {
                    let uuid = from_uuid(&vession);
                    quote!(LinkName::Version(#uuid))
                }
                LinkName::Card(card_id) => {
                    let uuid = from_uuid(&card_id);
                    quote!(LinkName::Card(#uuid))
                }
            };
            let arguments = arguments
                .into_iter()
                .map(|argument| from_expr(&argument))
                .collect::<Vec<_>>();

            quote! {
                Expr::Apply {
                    function: #function,
                    link_name: #link_name,
                    arguments: vec![#(#arguments),*],
                }
            }
        }
        Expr::Product(exprs) => {
            let exprs = exprs
                .into_iter()
                .map(|expr| from_expr(&expr))
                .collect::<Vec<_>>();
            quote! {
                Expr::Product(vec![#(#exprs),*])
            }
        }
        Expr::Match { of, cases } => {
            let of = from_expr(&*of);
            let cases = cases
                .into_iter()
                .map(|case| {
                    let ty = from_type_for_ast(&case.value.ty);
                    let expr = from_expr(&case.value.expr);
                    let tokens = quote! {
                        MatchCase {
                            ty: #ty,
                            expr: #expr,
                        }
                    };
                    with_meta(&case.meta, tokens)
                })
                .collect::<Vec<_>>();
            quote! {
                Expr::Match {
                    of: Box::new(#of),
                    cases: vec![#(#cases),*],
                }
            }
        }
        Expr::Typed { ty, item } => {
            let ty = from_type_for_ast(&ty);
            let item = from_expr(&*item);
            quote! {
                Expr::Typed {
                    ty: #ty,
                    item: Box::new(#item),
                }
            }
        }
        Expr::Hole => quote!(Expr::Hole),
        Expr::Function { parameter, body } => {
            let parameter = from_type_for_ast(&parameter);
            let body = from_expr(&*body);
            quote! {
                Expr::Function {
                    parameter: #parameter,
                    body: Box::new(#body),
                }
            }
        }
        Expr::Vector(exprs) => {
            let exprs = exprs
                .into_iter()
                .map(|expr| from_expr(&expr))
                .collect::<Vec<_>>();
            quote! {
                Expr::Vector(vec![#(#exprs),*])
            }
        }
        Expr::Map(elems) => {
            let elems = elems
                .into_iter()
                .map(|elem| {
                    let key = from_expr(&elem.value.key);
                    let value = from_expr(&elem.value.value);
                    let tokens = quote! {
                        MapElem {
                            key: Box::new(#key),
                            value: Box::new(#value),
                        }
                    };
                    with_meta(&elem.meta, tokens)
                })
                .collect::<Vec<_>>();
            quote! {
                Expr::Map(vec![#(#elems),*])
            }
        }
        Expr::Attributed { attr, item } => {
            let attr = from_dson(&attr);
            let item = from_expr(&*item);
            quote! {
                Expr::Attributed {
                    attr: #attr,
                    item: Box::new(#item),
                }
            }
        }
        Expr::DeclareBrand { brand, item } => {
            let item = from_expr(&*item);
            quote! {
                Expr::Brand {
                    brand: #brand.to_string(),
                    item: Box::new(#item),
                }
            }
        }
        Expr::Label { label, item } => {
            let item = from_expr(&*item);
            quote! {
                Expr::Label {
                    label: #label.to_string(),
                    item: Box::new(#item),
                }
            }
        }
        Expr::NewType { ident, ty, expr } => {
            let ty = from_type_for_ast(&ty);
            let expr = from_expr(&*expr);
            quote! {
                Expr::NewType {
                    ident: #ident.to_string(),
                    ty: #ty,
                    expr: Box::new(#expr),
                }
            }
        }
        Expr::Card { id, item, next } => {
            let id = from_uuid(&id.0);
            let item = from_expr(&*item);
            let next = from_expr(&*next);
            quote! {
                Expr::Card {
                    id: CardId(#id),
                    item: Box::new(#item),
                    next: Box::new(#next),
                }
            }
        }
    };
    with_meta(&expr.meta, tokens)
}

fn from_type_for_ast(ty: &WithMeta<ast::ty::Type>) -> proc_macro2::TokenStream {
    let tokens = match &ty.value {
        Type::Labeled { brand, item } => {
            let item = from_type_for_ast(&*item);
            quote! {
                Type::Labeled {
                    brand: #brand.to_string(),
                    item: Box::new(#item),
                }
            }
        }
        Type::Real => quote!(Type::Real),
        Type::Rational => quote!(Type::Rational),
        Type::Integer => quote!(Type::Integer),
        Type::String => quote!(Type::String),
        Type::Effectful { ty, effects } => {
            let ty = from_type_for_ast(&*ty);
            let effects = from_effect_expr_for_ast(&effects);
            quote! {
                Type::Effectful {
                    ty: Box::new(#ty),
                    effects: #effects,
                }
            }
        }
        Type::Infer => quote!(Type::Infer),
        Type::Product(types) => {
            let types = types.into_iter().map(from_type_for_ast).collect::<Vec<_>>();
            quote! {
                Type::Product(vec![#(#types),*])
            }
        }
        Type::Sum(types) => {
            let types = types.into_iter().map(from_type_for_ast).collect::<Vec<_>>();
            quote! {
                Type::Sum(vec![#(#types),*])
            }
        }
        Type::Function(function) => {
            let function = from_function(&*function);
            quote! {
                Type::Function(Box::new(#function))
            }
        }
        Type::Vector(ty) => {
            let ty = from_type_for_ast(&*ty);
            quote! {
                Type::Vector(Box::new(#ty))
            }
        }
        Type::Map { key, value } => {
            let key = from_type_for_ast(&*key);
            let value = from_type_for_ast(&*value);
            quote! {
                Type::Map {
                    key: Box::new(#key),
                    value: Box::new(#value),
                }
            }
        }
        Type::Let {
            variable,
            definition,
            body,
        } => {
            let definition = from_type_for_ast(&*definition);
            let body = from_type_for_ast(&*body);
            quote! {
                Type::Let {
                    variable: #variable.to_string(),
                    definition: Box::new(#definition),
                    body: Box::new(#body),
                }
            }
        }
        Type::Variable(ident) => quote!(Type::Variable(#ident.to_string())),
        Type::Attributed { attr, ty } => {
            let attr = from_dson(&attr);
            let ty = from_type_for_ast(&*ty);
            quote! {
                Type::Attributed {
                    attr: #attr,
                    ty: Box::new(#ty),
                }
            }
        }
        Type::Forall {
            variable,
            bound,
            body,
        } => {
            let bound = from_bound(&bound);
            let body = from_type_for_ast(&*body);
            quote! {
                Type::Forall {
                    variable: #variable.to_string(),
                    bound: #bound,
                    body: Box::new(#body),
                }
            }
        }
        Type::Exists {
            variable,
            bound,
            body,
        } => {
            let bound = from_bound(&bound);
            let body = from_type_for_ast(&*body);
            quote! {
                Type::Exists {
                    variable: #variable.to_string(),
                    bound: #bound,
                    body: Box::new(#body),
                }
            }
        }
    };
    with_meta(&ty.meta, tokens)
}

fn from_bound(bound: &Option<Box<WithMeta<Type>>>) -> proc_macro2::TokenStream {
    match bound {
        Some(bound) => {
            let tokens = from_type_for_ast(&*bound);
            quote!(Box::new(#tokens))
        }
        None => quote!(None),
    }
}

fn from_effect_expr_for_ast(expr: &WithMeta<EffectExpr>) -> proc_macro2::TokenStream {
    let tokens = match &expr.value {
        EffectExpr::Effects(effects) => {
            let effects = effects
                .into_iter()
                .map(from_effect_for_ast)
                .collect::<Vec<_>>();
            quote! {
                EffectExpr::Effects(vec![#(#effects),*])
            }
        }
        EffectExpr::Add(exprs) => {
            let exprs = exprs
                .into_iter()
                .map(from_effect_expr_for_ast)
                .collect::<Vec<_>>();
            quote! {
                EffectExpr::Add(vec![#(#exprs),*])
            }
        }
        EffectExpr::Sub {
            minuend,
            subtrahend,
        } => {
            let minuend = from_effect_expr_for_ast(&*minuend);
            let subtrahend = from_effect_expr_for_ast(&*subtrahend);
            quote! {
                EffectExpr::Sub {
                    minuend: Box::new(#minuend),
                    subtrahend: Box::new(#subtrahend),
                }
            }
        }
        EffectExpr::Apply {
            function,
            arguments,
        } => {
            let function = from_type_for_ast(&*function);
            let arguments = arguments
                .into_iter()
                .map(from_type_for_ast)
                .collect::<Vec<_>>();
            quote! {
                EffectExpr::Apply {
                    function: Box::new(#function),
                    arguments: vec![#(#arguments),*],
                }
            }
        }
    };
    with_meta(&expr.meta, tokens)
}

fn from_effect_for_ast(effect: &WithMeta<Effect>) -> proc_macro2::TokenStream {
    let input = from_type_for_ast(&effect.value.input);
    let output = from_type_for_ast(&effect.value.output);
    let tokens = quote! {
        Effect {
            input: #input,
            output: #output,
        }
    };
    with_meta(&effect.meta, tokens)
}

fn from_function(function: &Function) -> proc_macro2::TokenStream {
    let parameter = from_type_for_ast(&function.parameter);
    let body = from_type_for_ast(&function.body);
    quote! {
        Function {
            parameter: #parameter,
            body: #body,
        }
    }
}

fn from_uuid(uuid: &Uuid) -> proc_macro2::TokenStream {
    let string = uuid.to_string();
    quote!(uuid::uuid!(#string))
}

fn with_meta(meta: &Meta, value: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let id = from_uuid(&meta.id.0);
    let comments = meta
        .comments
        .before
        .iter()
        .map(|comment| match comment {
            Comment::Line(line) => quote!(Comment::Line(#line.to_string())),
            Comment::Block(block) => quote!(Comment::Block(#block.to_string())),
        })
        .collect::<Vec<_>>();
    let after = match &meta.comments.after {
        Some(string) => quote!(Some(#string.to_string())),
        None => quote!(None),
    };
    quote! {
        WithMeta {
            meta: Meta {
                id: NodeId(#id),
                comments: Comments {
                    before: vec![#(#comments),*],
                    after: #after,
                }
            },
            value: #value,
        }
    }
}
