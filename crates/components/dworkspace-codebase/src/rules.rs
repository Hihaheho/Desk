use std::collections::{HashMap, HashSet};

use types::Type;

use crate::user::UserId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rules<Operation: Eq + std::hash::Hash> {
    /// Used if user is not in the map.
    pub default: HashSet<Operation>,
    pub users: HashMap<UserId, HashSet<Operation>>,
}

impl<Operation: Eq + std::hash::Hash> Default for Rules<Operation> {
    fn default() -> Self {
        Self {
            default: Default::default(),
            users: Default::default(),
        }
    }
}

impl<T: Eq + std::hash::Hash + Clone> Rules<T> {
    pub fn user_has_operation(&self, user_id: &UserId, operation: &T) -> bool {
        self.users
            .get(user_id)
            .unwrap_or(&self.default)
            .contains(operation)
    }

    pub fn intersection(&self, other: &Rules<T>) -> Rules<T> {
        let default = self.default.intersection(&other.default).cloned().collect();
        let mut users: HashMap<UserId, _> = HashMap::new();
        for &user_id in self
            .users
            .keys()
            .collect::<HashSet<_>>()
            .intersection(&other.users.keys().collect::<HashSet<_>>())
        {
            let operations = self
                .users
                .get(user_id)
                .unwrap()
                .intersection(other.users.get(user_id).unwrap())
                .cloned()
                .collect();
            users.insert(user_id.clone(), operations);
        }
        Rules { default, users }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpaceOperation {
    AddOwner,
    RemoveOwner,
    AddSnapshot,
    CreateNode,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeOperation {
    RemoveNode,
    PatchSourceCode,
    ChangeSourceCodeSyntax,
    PatchString,
    UpdateInteger,
    UpdateReal,
    UpdateRational,
    UpdateApply,
    UpdateApplyLinkName,
    ReplaceContent,
    InsertOperand,
    RemoveOperand,
    MoveOperand,
    // These should be boxed for size reason?
    UpdateAttribute(Type),
    RemoveAttribute(Type),
    UpdateRules,
    UpdateOperandRules,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_denied() {
        let rules = Rules::default();
        assert!(!rules.user_has_operation(&UserId("a".into()), &SpaceOperation::AddOwner));
    }

    #[test]
    fn returns_allowed() {
        let mut rules = Rules::default();
        rules.default.insert(SpaceOperation::AddOwner);
        assert!(rules.user_has_operation(&UserId("a".into()), &SpaceOperation::AddOwner));
    }

    #[test]
    fn returns_allowed_for_user() {
        let mut rules = Rules::default();
        rules.users.insert(
            UserId("a".into()),
            [SpaceOperation::AddOwner].into_iter().collect(),
        );
        assert!(rules.user_has_operation(&UserId("a".into()), &SpaceOperation::AddOwner));
    }

    #[test]
    fn returns_intersection() {
        use NodeOperation::*;
        let a = Rules {
            default: [UpdateInteger, UpdateReal].into_iter().collect(),
            users: [
                (
                    UserId("a".into()),
                    [UpdateInteger, UpdateReal].into_iter().collect(),
                ),
                (
                    UserId("b".into()),
                    [UpdateInteger, ReplaceContent].into_iter().collect(),
                ),
                (UserId("c".into()), [UpdateInteger].into_iter().collect()),
            ]
            .into_iter()
            .collect(),
        };
        let b = Rules {
            default: [UpdateInteger, UpdateRules].into_iter().collect(),
            users: [
                (
                    UserId("a".into()),
                    [UpdateReal, ReplaceContent].into_iter().collect(),
                ),
                (
                    UserId("b".into()),
                    [UpdateReal, ReplaceContent].into_iter().collect(),
                ),
                (UserId("d".into()), [UpdateInteger].into_iter().collect()),
            ]
            .into_iter()
            .collect(),
        };
        assert_eq!(
            a.intersection(&b),
            Rules {
                default: [UpdateInteger].into_iter().collect(),
                users: [
                    (UserId("a".into()), [UpdateReal].into_iter().collect(),),
                    (UserId("b".into()), [ReplaceContent].into_iter().collect(),),
                ]
                .into_iter()
                .collect(),
            }
        );
    }
}
