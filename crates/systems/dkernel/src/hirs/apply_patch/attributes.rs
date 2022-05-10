use dkernel_card::{flat_node::Attributes, patch::AttributePatch};

use super::AttributePatchApplier;

impl AttributePatchApplier for Attributes {
    fn apply_patch(mut self, patch: &AttributePatch) -> Self {
        match patch {
            AttributePatch::Update { key, value } => {
                self.insert(key.clone(), *value.clone());
            }
            AttributePatch::Remove { key } => {
                self.remove(key);
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use deskc_hir::expr::{Expr, Literal};
    use deskc_types::Type;
    use dkernel_card::patch::AttributePatch;

    use super::*;

    #[test]
    fn update() {
        let attributes = Attributes::default();
        let attributes = attributes.apply_patch(&AttributePatch::Update {
            key: Type::Number,
            value: Box::new(Expr::Literal(Literal::Integer(1))),
        });
        assert_eq!(
            attributes.get(&Type::Number),
            Some(&Expr::Literal(Literal::Integer(1)))
        );
    }

    #[test]
    fn remove() {
        let attributes = Attributes::default();
        let attributes = attributes.apply_patch(&AttributePatch::Update {
            key: Type::Number,
            value: Box::new(Expr::Literal(Literal::Integer(1))),
        });
        let attributes = attributes.apply_patch(&AttributePatch::Remove { key: Type::Number });

        assert_eq!(attributes.get(&Type::Number), None,);
    }
}
