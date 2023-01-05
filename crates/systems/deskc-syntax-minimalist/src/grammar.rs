use crate::grammar_trait::{ExprC, GrammarTrait};
use parol_runtime::miette::Result;

///
/// Data structure that implements the semantic actions for our grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct Grammar<'t> {
    pub expr: Option<ExprC<'t>>,
}

impl Grammar<'_> {
    pub fn new() -> Self {
        Grammar::default()
    }
}

impl<'t> GrammarTrait<'t> for Grammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal '{{grammar_name}}'
    fn expr_c(&mut self, arg: &ExprC<'t>) -> Result<()> {
        self.expr = Some(arg.clone());
        Ok(())
    }
}
