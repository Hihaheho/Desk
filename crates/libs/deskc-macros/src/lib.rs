use ast::{
    expr::Expr,
    span::WithSpan,
    ty::{Effect, EffectExpr, Function, Type},
};
use dson::{Dson, MapElem};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn ty(item: TokenStream) -> TokenStream {
    fn input(string: String) -> String {
        "&".to_string() + &string
    }
    fn map(expr: WithSpan<Expr>) -> proc_macro2::TokenStream {
        if let Expr::Apply { function, .. } = expr.value {
            from_type(function.value)
        } else {
            panic!("Failed to parse reference")
        }
    }
    parse(item, input, map)
}

#[proc_macro]
pub fn effect(item: TokenStream) -> TokenStream {
    fn input(string: String) -> String {
        format!("& ! {{ {string} }} 'integer")
    }
    fn map(expr: WithSpan<Expr>) -> proc_macro2::TokenStream {
        let Expr::Apply { function, .. } = expr.value
         else {
            panic!("Failed to parse reference")
        };
        let Type::Effectful { ty: _, effects } = function.value else {
            panic!("Failed to parse effectful")
        };
        let EffectExpr::Effects(effects) = effects.value else {
            panic!("Failed to parse effects")
        };
        from_effect(effects[0].value.clone())
    }
    parse(item, input, map)
}

#[proc_macro]
pub fn dson(item: TokenStream) -> TokenStream {
    fn input(string: String) -> String {
        format!("@ {string} 1")
    }
    fn map(expr: WithSpan<Expr>) -> proc_macro2::TokenStream {
        let Expr::Label { label, item: _ } = expr.value else {
            panic!("Failed to parse label")
        };
        from_dson(label)
    }
    parse(item, input, map)
}

fn parse(
    item: TokenStream,
    input: fn(String) -> String,
    map: fn(WithSpan<Expr>) -> proc_macro2::TokenStream,
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
        match parser::parse(&input(string)) {
            Ok(expr) => {
                let tokens = map(expr);
                quote! {
                    {
                        use ty::{Effect, Function, Type, EffectExpr};
                        use dson::{Dson, Literal};
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

fn from_type(ty: Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Labeled { brand, item } => {
            let brand = from_dson(brand);
            let item = from_type(item.value);
            quote! {
                Type::Label {
                    label: #brand,
                    item: Box::new(#item),
                }
            }
        }
        Type::Real => quote! { Type::Real },
        Type::Rational => quote! { Type::Rational },
        Type::Integer => quote! { Type::Integer },
        Type::String => quote! { Type::String },
        Type::Trait(_) => todo!(),
        Type::Effectful { ty, effects } => {
            let ty = from_type(ty.value);
            let effects = from_effect_expr(effects.value);
            quote! {
                Type::Effectful {
                    ty: Box::new(#ty),
                    effects: #effects,
                }
            }
        }
        Type::Infer => quote! { Type::Infer },
        Type::This => quote! { Type::This },
        Type::Product(types) => {
            let types = types
                .into_iter()
                .map(|ty| from_type(ty.value))
                .collect::<Vec<_>>();
            quote! {
                Type::Product(vec![#(#types),*])
            }
        }
        Type::Sum(types) => {
            let types = types
                .into_iter()
                .map(|ty| from_type(ty.value))
                .collect::<Vec<_>>();
            quote! {
                Type::Sum(vec![#(#types),*])
            }
        }
        Type::Function(function) => {
            let Function { parameter, body } = *function;
            let parameter = from_type(parameter.value);
            let body = from_type(body.value);
            quote! {
                Type::Function(Box::new(Function {
                    parameter: #parameter,
                    body: #body,
                }))
            }
        }
        Type::Vector(ty) => {
            let ty = from_type(ty.value);
            quote! {
                Type::Vector(Box::new(#ty))
            }
        }
        Type::Map { key, value } => {
            let key = from_type(key.value);
            let value = from_type(value.value);
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
            let definition = from_type(definition.value);
            let body = from_type(body.value);
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
            let attr = from_dson(attr);
            let ty = from_type(ty.value);
            quote! {
                Type::Attributed {
                    attr: #attr,
                    ty: Box::new(#ty),
                }
            }
        }
        Type::Comment { text, item } => {
            let item = from_type(item.value);
            quote! {
                Type::Comment {
                    text: #text.into(),
                    item: Box::new(#item),
                }
            }
        }
        Type::Forall {
            variable,
            bound,
            body,
        } => {
            let bound = if let Some(bound) = bound {
                let bound = from_type(bound.value);
                quote! { Some(Box::new(#bound)) }
            } else {
                quote! { None }
            };
            let body = from_type(body.value);
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
                let bound = from_type(bound.value);
                quote! { Some(Box::new(#bound)) }
            } else {
                quote! { None }
            };
            let body = from_type(body.value);
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

fn from_dson(dson: Dson) -> proc_macro2::TokenStream {
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
                    let key = from_dson(key);
                    let value = from_dson(value);
                    quote! { (#key, #value) }
                })
                .collect::<Vec<_>>();
            quote! {
                Dson::Map(vec![#(#elems),*])
            }
        }
        Dson::Attributed { attr, expr } => {
            let attr = from_dson(*attr);
            let expr = from_dson(*expr);
            quote! {
                Dson::Attributed {
                    attr: Box::new(#attr),
                    expr: Box::new(#expr),
                }
            }
        }
        Dson::Labeled { label, expr } => {
            let label = from_dson(*label);
            let expr = from_dson(*expr);
            quote! {
                Dson::Labeled {
                    label: Box::new(#label),
                    expr: Box::new(#expr),
                }
            }
        }
        Dson::Typed { ty, expr } => {
            let ty = from_dson_type(ty);
            let expr = from_dson(*expr);
            quote! {
                Dson::Typed {
                    ty: Box::new(#ty),
                    expr: Box::new(#expr),
                }
            }
        }
        Dson::Comment { text, expr } => {
            let expr = from_dson(*expr);
            quote! {
                Dson::Comment {
                    text: #text.into(),
                    expr: Box::new(#expr),
                }
            }
        }
    }
}

fn from_effect_expr(expr: EffectExpr) -> proc_macro2::TokenStream {
    match expr {
        EffectExpr::Effects(effects) => {
            let effects = effects
                .into_iter()
                .map(|effect| from_effect(effect.value))
                .collect::<Vec<_>>();
            quote! {
                EffectExpr::Effects(vec![#(#effects),*])
            }
        }
        EffectExpr::Add(exprs) => {
            let exprs = exprs
                .into_iter()
                .map(|expr| from_effect_expr(expr.value))
                .collect::<Vec<_>>();
            quote! {
                EffectExpr::Add(vec![#(#exprs),*])
            }
        }
        EffectExpr::Sub {
            minuend,
            subtrahend,
        } => {
            let minuend = from_effect_expr(minuend.value);
            let subtrahend = from_effect_expr(subtrahend.value);
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
            let function = from_type(function.value);
            let arguments = arguments
                .into_iter()
                .map(|argument| from_type(argument.value))
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

fn from_effect(effect: Effect) -> proc_macro2::TokenStream {
    let input = from_type(effect.input.value);
    let output = from_type(effect.output.value);
    quote! {
        Effect {
            input: #input,
            output: #output,
        }
    }
}

fn from_dson_type(ty: dson::Type) -> proc_macro2::TokenStream {
    match ty {
        dson::Type::Brand { brand, item } => {
            let brand = from_dson(*brand);
            let item = from_dson_type(*item);
            quote! {
                dson::Type::Brand {
                    brand: Box::new(#brand),
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
            let ty = from_dson_type(*ty);
            quote! {
                dson::Type::Vector(Box::new(#ty))
            }
        }
        dson::Type::Map { key, value } => {
            let key = from_dson_type(*key);
            let value = from_dson_type(*value);
            quote! {
                dson::Type::Map {
                    key: Box::new(#key),
                    value: Box::new(#value),
                }
            }
        }
        dson::Type::Attributed { attr, ty } => {
            let attr = from_dson(*attr);
            let ty = from_dson_type(*ty);
            quote! {
                dson::Type::Attributed {
                    attr: Box::new(#attr),
                    ty: Box::new(#ty),
                }
            }
        }
        dson::Type::Comment { text, item } => {
            let item = from_dson_type(*item);
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
            let definition = from_dson_type(*definition);
            let body = from_dson_type(*body);
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
