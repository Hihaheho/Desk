mod diff_match_patch;
use deskc_ids::{LinkName, NodeId};
use dson::Dson;
use ty::Type;

use crate::{code::SyntaxKind, content::Content};

use self::diff_match_patch::Patch;

#[derive(Debug, Clone, PartialEq)]
pub enum ContentPatch {
    Replace(Content),
    ChangeSourceCodeSyntax { syntax: SyntaxKind, source: String },
    PatchSourceCode(StringPatch),
    PatchString(StringPatch),
    UpdateInteger(u64),
    UpdateReal(f64),
    UpdateRational(u64, u64),
    UpdateApply { ty: Type, link_name: LinkName },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringPatch {
    Replace(String),
    DiffMatchPatch(Vec<Patch>),
}

// ContentPatch::AddReal should not be NaN
impl Eq for ContentPatch {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperandPatch {
    Insert { index: usize, node_id: NodeId },
    Remove { index: usize },
    Move { from: usize, to: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributePatch {
    Update { key: Type, value: Dson },
    Remove { key: Type },
}
