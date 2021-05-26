use std::ops::Add;

use super::NumberLiteral::*;

impl Add for super::NumberLiteral {
    type Output = Self;
    fn add(self, other: Self) -> Self {
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
        assert_eq!(Integer(1) + Integer(2), Integer(3));
    }
}
