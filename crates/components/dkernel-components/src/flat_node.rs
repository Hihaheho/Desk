use std::collections::HashMap;

use deskc_ids::{FileId, NodeId};
use hir::expr::Expr;
use types::Type;

use crate::content::Content;

pub type Children = Vec<NodeId>;
pub type Attributes = HashMap<Type, Expr>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlatNode {
    pub file_id: FileId,
    pub content: Content,
    pub children: Children,
    pub attributes: Attributes,
}
