mod string_diff;
use deskc_ids::NodeId;
use hir::expr::Expr;
use types::Type;

use crate::content::Content;

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
pub enum OperandsPatch {
    Insert { index: usize, node: NodeId },
    Remove { index: usize },
    Move { index: usize, diff: isize },
    Update { index: usize, node: NodeId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributePatch {
    Update { key: Type, value: Box<Expr> },
    Remove { key: Type },
}
