use ast::{expr::Expr, span::Spanned};

pub fn extract_includes(src: &Spanned<Expr>) -> Vec<String> {
    let mut includes = Vec::new();
    visit(&mut includes, &src.0);
    includes
}

fn visit(includes: &mut Vec<String>, expr: &Expr) {
    match expr {
        Expr::Include(path) => includes.push(path.clone()),
        Expr::Let {
            ty: _,
            definition,
            expression,
        } => {
            visit(includes, &definition.0);
            visit(includes, &expression.0);
        }
        Expr::Perform { input, output: _ } => visit(includes, &input.0),
        Expr::Handle {
            input: _,
            output: _,
            handler,
            expr,
        } => {
            visit(includes, &handler.0);
            visit(includes, &expr.0);
        }
        Expr::Apply {
            function: _,
            arguments,
        } => {
            for arg in arguments {
                visit(includes, &arg.0);
            }
        }
        Expr::Product(exprs) => {
            for expr in exprs {
                visit(includes, &expr.0);
            }
        }
        Expr::Match { of, cases } => {
            visit(includes, &of.0);
            for case in cases {
                visit(includes, &case.expr.0);
            }
        }
        Expr::Typed { ty: _, item: expr } => visit(includes, &expr.0),
        Expr::Hole => {}
        Expr::Function {
            parameters: _,
            body,
        } => visit(includes, &body.0),
        Expr::Array(exprs) => {
            for expr in exprs {
                visit(includes, &expr.0);
            }
        }
        Expr::Set(exprs) => {
            for expr in exprs {
                visit(includes, &expr.0);
            }
        }
        Expr::Attribute { attr, item: expr } => {
            visit(includes, &attr.0);
            visit(includes, &expr.0);
        }
        Expr::Brand {
            brands: _,
            item: expr,
        } => visit(includes, &expr.0),
        Expr::Import { ty: _, uuid: _ } => todo!(),
        Expr::Export { ty: _ } => todo!(),
        Expr::Literal(_) => {}
        Expr::Label {
            label: _,
            item: expr,
        } => visit(includes, &expr.0),
        Expr::NewType {
            ident: _,
            ty: _,
            expr,
        } => visit(includes, &expr.0),
    };
}
