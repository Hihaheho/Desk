use components::patch::{AttributePatch, ChildrenPatch, ContentPatch};

mod attributes;
mod children;
mod content;

pub(super) trait ContentPatchApplier {
    fn apply_patch(self, patch: &ContentPatch) -> Self;
}

pub(super) trait ChildrenPatchApplier {
    fn apply_patch(self, patch: &ChildrenPatch) -> Self;
}

pub(super) trait AttributePatchApplier {
    fn apply_patch(self, patch: &AttributePatch) -> Self;
}
