use std::collections::HashMap;

use ast::parser::{dyn_eq, SpanStorage};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MinimalistSyntaxSpanStorage {
    spans: HashMap<ids::NodeId, parol_runtime::Span>,
}

impl SpanStorage for MinimalistSyntaxSpanStorage {
    fn calculate_span(&self, id: &ids::NodeId) -> Option<ast::meta::Span> {
        self.spans.get(&id).map(|s| s.into())
    }
    fn dyn_eq(&self, other: &dyn SpanStorage) -> bool {
        dyn_eq(self, other)
    }
}

impl From<(ids::NodeId, Option<&crate::grammar_trait::ExprC<'_>>)> for MinimalistSyntaxSpanStorage {
    fn from(value: (ids::NodeId, Option<&crate::grammar_trait::ExprC<'_>>)) -> Self {
        use crate::grammar_trait::Expr;
        use parol_runtime::lexer::rng::ToSpan;
        if let Some(expr) = value.1 {
            let mut spans = HashMap::new();
            spans.insert(
                value.0,
                match &*expr.expr {
                    Expr::ExprBeginExprCExprEnd(e) => ToSpan::span(e),
                    Expr::Hole(e) => ToSpan::span(e),
                    Expr::Do(e) => ToSpan::span(e),
                    Expr::Cast(e) => ToSpan::span(e),
                    Expr::Literal(e) => ToSpan::span(e),
                    Expr::Let(e) => ToSpan::span(e),
                    Expr::Perform(e) => ToSpan::span(e),
                    Expr::Continue(e) => ToSpan::span(e),
                    Expr::Handle(e) => ToSpan::span(e),
                    Expr::Product(e) => ToSpan::span(e),
                    Expr::Vector(e) => ToSpan::span(e),
                    Expr::Map(e) => ToSpan::span(e),
                    Expr::Attributed(e) => ToSpan::span(e),
                    Expr::Match(e) => ToSpan::span(e),
                    Expr::Function(e) => ToSpan::span(e),
                    Expr::Apply(e) => ToSpan::span(e),
                    Expr::Reference(e) => ToSpan::span(e),
                    Expr::Forall(e) => ToSpan::span(e),
                    Expr::Exists(e) => ToSpan::span(e),
                    Expr::Labeled(e) => ToSpan::span(e),
                    Expr::NewType(e) => ToSpan::span(e),
                    Expr::Card(e) => ToSpan::span(e),
                    Expr::Brand(e) => ToSpan::span(e),
                },
            );
            Self { spans }
        } else {
            Self::default()
        }
    }
}
