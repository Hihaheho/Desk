use components::{content::Content, patch::ContentPatch};

use super::ContentPatchApplier;

impl ContentPatchApplier for &Content {
    fn apply_patch(self, patch: &ContentPatch) -> Content {
        match patch {
            ContentPatch::Replace(content) => content.clone(),
            ContentPatch::PatchString(_) => todo!(),
            ContentPatch::AddInteger(_) => todo!(),
            ContentPatch::AddFloat(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {}
