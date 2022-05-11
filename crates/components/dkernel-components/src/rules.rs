use std::collections::{HashMap, HashSet};

use deskc_ids::UserId;

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

impl<T: Eq + std::hash::Hash> Rules<T> {
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
    AddFile,
    DeleteFile,
    PatchFile,
    UpdateRule,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeOperation {
    AddCard,
    RemoveCard,
    AddNode,
    RemoveNode,
    PatchContentReplace,
    PatchContentPatchString,
    PatchContentAddInteger,
    PatchContentAddFloat,
    PatchChildrenInsert,
    PatchChildrenRemove,
    PatchChildrenMove,
    PatchChildrenUpdate,
    PatchAttributeUpdate,
    PatchAttributeRemove,
    PatchFileUpdateRules,
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
}
