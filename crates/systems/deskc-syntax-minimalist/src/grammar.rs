use crate::grammar_trait::{Expr, GrammarTrait};
use anyhow::Result;

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
