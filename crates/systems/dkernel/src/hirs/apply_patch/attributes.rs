use components::{flat_node::Attributes, patch::AttributePatch};

use super::AttributePatchApplier;

impl AttributePatchApplier for &Attributes {
    fn apply_patch(self, patch: &AttributePatch) -> Attributes {
        let mut attributes = self.clone();
        match patch {
            AttributePatch::Update { key, value } => {
                attributes.insert(key.clone(), *value.clone());
            }
            AttributePatch::Remove { key } => {
                attributes.remove(key);
            }
        }
        attributes
    }
}

#[cfg(test)]
mod tests {
    use components::patch::AttributePatch;
    use deskc_hir::expr::{Expr, Literal};
    use deskc_types::Type;

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
