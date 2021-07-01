use super::node::Node;

pub struct TypingError {}

pub fn typing(_node: &mut Node) -> Result<(), TypingError> {
    todo!()
}

#[cfg(test)]
mod test_literal;
