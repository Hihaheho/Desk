use crate::processor_attachment::ProcessorAttachment;

use super::DProcess;

impl DProcess {
    pub fn update_processor_attachment(&self, new_processor_attachment: ProcessorAttachment) {
        *self.processor_attachment.write() = new_processor_attachment;
    }
}
