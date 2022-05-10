use dkernel_card::{content::Content, patch::ContentPatch};

use super::ContentPatchApplier;

impl ContentPatchApplier for Content {
    fn apply_patch(mut self, patch: &ContentPatch) -> Self {
        match patch {
            ContentPatch::Replace(content) => self = content.clone(),
            ContentPatch::PatchString(_) => {}
            ContentPatch::AddInteger(_) => {}
            ContentPatch::AddFloat(_) => {}
        }
        self
    }
}

#[cfg(test)]
mod tests {}
