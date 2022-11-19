use parol_runtime::derive_builder;

pub mod grammar_trait {
    #![allow(unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/grammar_trait.rs"));
}
pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}
mod grammar {
    use crate::grammar_trait::{Expr, GrammarTrait};
    #[allow(unused_imports)]
    use parol_runtime::miette::Result;

    ///
    /// Data structure that implements the semantic actions for our grammar
    /// !Change this type as needed!
    ///
    #[derive(Debug, Default)]
    pub struct Grammar<'t> {
        pub expr: Option<Expr<'t>>,
    }

    impl Grammar<'_> {
        pub fn new() -> Self {
            Grammar::default()
        }
    }

    impl<'t> GrammarTrait<'t> for Grammar<'t> {
        // !Adjust your implementation as needed!

        /// Semantic action for non-terminal '{{grammar_name}}'
        fn expr(&mut self, arg: &Expr<'t>) -> Result<()> {
            self.expr = Some(arg.clone());
            Ok(())
        }
    }
}

use ast::{expr::Expr, span::WithSpan};

pub fn parse(input: &str) -> anyhow::Result<WithSpan<Expr>> {
    let mut grammar = grammar::Grammar::new();
    parser::parse(input, "dummy", &mut grammar).map_err(|err| anyhow::anyhow!("{}", err))?;
    // let expr = grammar.expr.unwrap().into();

    todo!()
}

impl From<grammar_trait::Expr<'_>> for WithSpan<Expr> {
    fn from(expr: grammar_trait::Expr) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}