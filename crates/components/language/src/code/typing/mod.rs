use super::node::Code;

pub struct TypingError {}

pub fn typing(_node: &mut Code) -> Result<(), TypingError> {
    todo!()
}

#[cfg(test)]
mod test_literal;
