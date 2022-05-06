mod string_diff;
use hir::expr::Expr;
use types::Type;

use crate::{content::Content, flat_node::NodeRef};

use self::string_diff::StringPatch;

#[derive(Debug, Clone, PartialEq)]
pub enum ContentPatch {
    Overwrite(Content),
    PatchString(Vec<StringPatch>),
    AddInteger(u64),
    AddFloat(f64),
}

// ContentPatch::AddFloat should not be NaN
impl Eq for ContentPatch {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChildrenPatch {
    Insert { index: usize, node: NodeRef },
    Remove(usize),
    Move { index: usize, diff: isize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributePatch {
    pub key: Type,
    pub value: Expr,
}
