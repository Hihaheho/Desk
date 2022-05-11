use components::{
    content::Content,
    flat_node::{Attributes, Children},
    patch::{AttributePatch, ChildrenPatch, ContentPatch},
};

mod attributes;
mod children;
mod content;

pub(super) trait ContentPatchApplier {
    fn apply_patch(self, patch: &ContentPatch) -> Content;
}

pub(super) trait ChildrenPatchApplier {
    fn apply_patch(self, patch: &ChildrenPatch) -> Children;
}

pub(super) trait AttributePatchApplier {
    fn apply_patch(self, patch: &AttributePatch) -> Attributes;
}
