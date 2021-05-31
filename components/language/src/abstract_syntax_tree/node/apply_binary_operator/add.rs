use crate::abstract_syntax_tree::node::NumberLiteral;
use NumberLiteral::*;

impl NumberLiteral {
    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Integer(left), Integer(right)) => Integer(left + right),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn add_integers() {
        assert_eq!(Integer(1).add(&Integer(2)), Integer(3));
    }
}
