mod string_diff;
use hir::expr::Expr;

use crate::{content::Content, flat_node::NodeRef};

use self::string_diff::StringPatch;

pub enum ContentPatch {
    Overwrite(Content),
    PatchString(Vec<StringPatch>),
    AddInteger(u64),
    AddFloat(f64),
}

pub enum ChildrenPatch {
    Insert { index: usize, node: NodeRef },
    Remove(usize),
    Move { index: usize, diff: isize },
}

pub struct AttributePatch {
    pub key: String,
    pub value: Expr,
}
