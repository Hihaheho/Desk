use language::code::node::NumberLiteral;

use super::component::InputId;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum WidgetEvent {
    UpdateString { id: InputId, value: String },
    UpdateNumber { id: InputId, value: NumberLiteral },
    LostFocus { id: InputId },
}
