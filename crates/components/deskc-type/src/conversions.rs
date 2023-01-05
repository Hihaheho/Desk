use dson::Dson;

pub trait DsonTypeDeduction {
    fn deduct_type(&self) -> crate::Type;
}

impl DsonTypeDeduction for Dson {
    fn deduct_type(&self) -> crate::Type {
        match self {
            Dson::Literal(literal) => match literal {
                dson::Literal::String(_) => crate::Type::String,
                dson::Literal::Integer(_) => crate::Type::Integer,
                dson::Literal::Rational(_, _) => crate::Type::Rational,
                dson::Literal::Real(_) => crate::Type::Real,
            },
            Dson::Product(dsons) => todo!(),
            Dson::Vector(dsons) => todo!(),
            Dson::Map(elems) => todo!(),
            Dson::Attributed { attr, expr } => todo!(),
            Dson::Labeled { label, expr } => todo!(),
            Dson::Typed { ty, expr } => todo!(),
            Dson::Comment { text, expr } => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use dson::Real;

    use super::*;

    #[test]
    fn test_dson_type_string() {
        let dson = Dson::Literal(dson::Literal::String("hello".to_string()));
        assert_eq!(dson.deduct_type(), crate::Type::String);
    }

    #[test]
    fn test_dson_type_integer() {
        let dson = Dson::Literal(dson::Literal::Integer(42));
        assert_eq!(dson.deduct_type(), crate::Type::Integer);
    }

    #[test]
    fn test_dson_type_rational() {
        let dson = Dson::Literal(dson::Literal::Rational(1, 2));
        assert_eq!(dson.deduct_type(), crate::Type::Rational);
    }

    #[test]
    fn test_dson_type_real() {
        let dson = Dson::Literal(dson::Literal::Real(Real(3.14)));
        assert_eq!(dson.deduct_type(), crate::Type::Real);
    }
}
