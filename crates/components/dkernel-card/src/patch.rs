mod string_diff;
use hir::expr::Expr;
use types::Type;

use crate::{
    content::Content,
    flat_node::NodeRef,
    rules::{NodeOperation, Rules},
};

use self::string_diff::StringPatch;

#[derive(Debug, Clone, PartialEq)]
pub enum ContentPatch {
    Replace(Content),
    PatchString(Vec<StringPatch>),
    AddInteger(u64),
    AddFloat(f64),
}

// ContentPatch::AddFloat should not be NaN
impl Eq for ContentPatch {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChildrenPatch {
    Insert { index: usize, node: NodeRef },
    Remove { index: usize },
    Move { index: usize, diff: isize },
    Update { index: usize, node: NodeRef },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributePatch {
    Update { key: Type, value: Expr },
    Remove { key: Type },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilePatch {
    UpdateRules { rules: Rules<NodeOperation> },
}
