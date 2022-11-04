use std::collections::{HashMap, HashSet};

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
    pub fn audit(&self, user_id: &UserId, operation: &T) -> AuditResponse {
        if self
            .users
            .get(user_id)
            .unwrap_or(&self.default)
            .contains(operation)
        {
            AuditResponse::Allowed
        } else {
            AuditResponse::Denied
        }
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

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum AuditResponse {
    Allowed,
    Denied,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpaceOperation {
    AddOwner,
    RemoveOwner,
    AddSnapshot,
    AddNode,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeOperation {
    AddNode,
    RemoveNode,
    PatchContentReplace,
    PatchContentPatchString,
    PatchContentAddInteger,
    PatchContentAddFloat,
    PatchOperandsInsert,
    PatchOperandsRemove,
    PatchOperandsMove,
    PatchOperandsUpdate,
    PatchAttributeUpdate,
    PatchAttributeRemove,
    UpdateRules,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_denied() {
        let rules = Rules::default();
        assert_eq!(
            rules.audit(&UserId("a".into()), &SpaceOperation::AddOwner),
            AuditResponse::Denied
        );
    }

    #[test]
    fn returns_allowed() {
        let mut rules = Rules::default();
        rules.default.insert(SpaceOperation::AddOwner);
        assert_eq!(
            rules.audit(&UserId("a".into()), &SpaceOperation::AddOwner),
            AuditResponse::Allowed
        );
    }

    #[test]
    fn returns_allowed_for_user() {
        let mut rules = Rules::default();
        rules.users.insert(
            UserId("a".into()),
            [SpaceOperation::AddOwner].into_iter().collect(),
        );
        assert_eq!(
            rules.audit(&UserId("a".into()), &SpaceOperation::AddOwner),
            AuditResponse::Allowed
        );
    }

    #[test]
    fn returns_intersection() {
        use NodeOperation::*;
        let a = Rules {
            default: [AddNode, RemoveNode].into_iter().collect(),
            users: [
                (
                    UserId("a".into()),
                    [AddNode, RemoveNode].into_iter().collect(),
                ),
                (
                    UserId("b".into()),
                    [AddNode, PatchContentReplace].into_iter().collect(),
                ),
                (UserId("c".into()), [AddNode].into_iter().collect()),
            ]
            .into_iter()
            .collect(),
        };
        let b = Rules {
            default: [AddNode, UpdateRules].into_iter().collect(),
            users: [
                (
                    UserId("a".into()),
                    [RemoveNode, PatchContentReplace].into_iter().collect(),
                ),
                (
                    UserId("b".into()),
                    [RemoveNode, PatchContentReplace].into_iter().collect(),
                ),
                (UserId("d".into()), [AddNode].into_iter().collect()),
            ]
            .into_iter()
            .collect(),
        };
        assert_eq!(
            a.intersection(&b),
            Rules {
                default: [AddNode].into_iter().collect(),
                users: [
                    (UserId("a".into()), [RemoveNode].into_iter().collect(),),
                    (
                        UserId("b".into()),
                        [PatchContentReplace].into_iter().collect(),
                    ),
                ]
                .into_iter()
                .collect(),
            }
        );
    }
}
