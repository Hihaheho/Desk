mod diff_match_patch;
use deskc_ids::{LinkName, NodeId};
use hir::expr::Expr;
use types::Type;

use crate::{code::SyntaxKind, content::Content};

use self::diff_match_patch::Patch;

#[derive(Debug, Clone, PartialEq)]
pub enum ContentPatch {
    Replace(Content),
    ChangeSourceCodeSyntax { syntax: SyntaxKind, source: String },
    PatchSourceCode(StringPatch),
    PatchString(StringPatch),
    UpdateInteger(u64),
    UpdateFloat(f64),
    UpdateRational(u64, u64),
    UpdateApply { ty: Type, link_name: LinkName },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringPatch {
    Replace(String),
    DiffMatchPatch(Vec<Patch>),
}

// ContentPatch::AddFloat should not be NaN
impl Eq for ContentPatch {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperandPatch {
    Insert { index: usize, node_id: NodeId },
    Remove { index: usize },
    Move { from: usize, to: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributePatch {
    Update { key: Type, value: Box<Expr> },
    Remove { key: Type },
}
