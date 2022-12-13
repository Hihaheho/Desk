use std::sync::Arc;

use ty::{
    conclusion::{CastStrategy, TypeConclusions, TypeToType},
    Type,
};

use crate::value::Value;

use super::EvalCfg;

impl EvalCfg {
    pub(crate) fn cast(
        conclusion: Arc<TypeConclusions>,
        value: &Value,
        ty: &Type,
        target: &Type,
    ) -> Value {
        if let Some(strategy) = conclusion.cast_strategies.get(&TypeToType {
            // FIXME: This clone is really bad.
            from: ty.clone(),
            to: target.clone(),
        }) {
            match strategy {
                CastStrategy::ProductToProduct(mapping) => {
                    let Value::Product(fields) = value else { panic!("Expected product but {:?}", value) };
                    Value::Product(
                        mapping
                            .iter()
                            .map(|(from, to)| {
                                let field = fields
                                    .get(from)
                                    .expect(&format!("Missing field {:?}", from));
                                let field = Self::cast(conclusion.clone(), field, from, to);
                                (to.clone(), field.clone())
                            })
                            .collect(),
                    )
                }
                CastStrategy::SumToSum(mapping) => {
                    let Value::Variant { ty: from, value } = value else { panic!("Expected variant but {:?}", value) };
                    let to = mapping
                        .get(from)
                        .expect(&format!("Missing variant {:?}", from));
                    let value = Self::cast(conclusion.clone(), value, from, to);
                    Value::Variant {
                        ty: to.clone(),
                        value: Box::new(value),
                    }
                }
                CastStrategy::ProductToInner(ty) => {
                    let Value::Product(fields) = value else { panic!("Expected product but {:?}", value) };
                    let value = fields.get(ty).expect(&format!("Missing field {:?}", ty));
                    Self::cast(conclusion.clone(), value, ty, target)
                }
                CastStrategy::InnerToSum(to) => Value::Variant {
                    ty: to.clone(),
                    value: Box::new(Self::cast(conclusion.clone(), value, ty, to)),
                },
            }
        } else {
            value.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use ty::conclusion::{CastStrategy, TypeToType};

    use super::*;

    #[test]
    fn test_cast_same() {
        let conclusion = Default::default();
        let value = Value::Int(1);
        let ty = Type::Integer;
        let target = Type::Integer;
        let ret = EvalCfg::cast(Arc::new(conclusion), &value, &ty, &target);
        assert_eq!(ret, Value::Int(1));
    }

    #[test]
    fn test_enum_variant() {
        let mut conclusion = TypeConclusions::default();
        let value = Value::Int(1);
        let ty = Type::Integer;
        let variant = Type::Label {
            label: "a".into(),
            item: Box::new(Type::Integer),
        };
        let target = Type::Sum(vec![variant.clone(), Type::Real]);
        conclusion.cast_strategies.insert(
            TypeToType {
                from: ty.clone(),
                to: target.clone(),
            },
            CastStrategy::InnerToSum(variant.clone()),
        );
        let ret = EvalCfg::cast(Arc::new(conclusion), &value, &ty, &target);
        assert_eq!(
            ret,
            Value::Variant {
                ty: variant,
                value: Box::new(Value::Int(1))
            }
        );
    }

    #[test]
    fn test_sum_to_sum() {
        let mut conclusion = TypeConclusions::default();
        let value = Value::Variant {
            ty: Type::Integer,
            value: Box::new(Value::Int(1)),
        };
        let ty = Type::Sum(vec![Type::Integer, Type::Real]);
        let target = Type::Sum(vec![Type::Rational, Type::Real]);
        conclusion.cast_strategies.insert(
            TypeToType {
                from: ty.clone(),
                to: target.clone(),
            },
            CastStrategy::SumToSum([(Type::Integer, Type::Rational)].into_iter().collect()),
        );
        let ret = EvalCfg::cast(Arc::new(conclusion), &value, &ty, &target);
        assert_eq!(
            ret,
            Value::Variant {
                ty: Type::Rational,
                value: Box::new(Value::Int(1))
            }
        );
    }

    #[test]
    fn test_product_to_type() {
        let mut conclusion = TypeConclusions::default();
        let value = Value::Product(
            [
                (Type::Integer, Value::Int(1)),
                (Type::Rational, Value::Int(2)),
            ]
            .into_iter()
            .collect(),
        );
        let ty = Type::Product(vec![Type::Integer, Type::Rational]);
        let target = Type::Rational;
        conclusion.cast_strategies.insert(
            TypeToType {
                from: ty.clone(),
                to: target.clone(),
            },
            CastStrategy::ProductToInner(Type::Rational),
        );
        let ret = EvalCfg::cast(Arc::new(conclusion), &value, &ty, &target);
        assert_eq!(ret, Value::Int(2));
    }

    #[test]
    fn test_product_to_product() {
        let mut conclusion = TypeConclusions::default();
        let value = Value::Product(
            [
                (Type::Integer, Value::Int(1)),
                (Type::Rational, Value::Int(2)),
            ]
            .into_iter()
            .collect(),
        );
        let ty = Type::Product(vec![Type::Integer, Type::Rational]);
        let target = Type::Product(vec![Type::Integer, Type::Real]);
        conclusion.cast_strategies.insert(
            TypeToType {
                from: ty.clone(),
                to: target.clone(),
            },
            CastStrategy::ProductToProduct(
                [(Type::Integer, Type::Integer), (Type::Rational, Type::Real)]
                    .into_iter()
                    .collect(),
            ),
        );
        let ret = EvalCfg::cast(Arc::new(conclusion), &value, &ty, &target);
        assert_eq!(
            ret,
            Value::Product(
                [(Type::Integer, Value::Int(1)), (Type::Real, Value::Int(2)),]
                    .into_iter()
                    .collect()
            )
        );
    }

    #[test]
    fn test_sum_to_sum_recursion() {
        let mut conclusion = TypeConclusions::default();
        let value = Value::Variant {
            ty: Type::Product(vec![Type::Integer, Type::Rational]),
            value: Box::new(Value::Product(
                [
                    (Type::Integer, Value::Int(1)),
                    (Type::Rational, Value::Int(2)),
                ]
                .into_iter()
                .collect(),
            )),
        };
        let ty = Type::Sum(vec![
            Type::Product(vec![Type::Integer, Type::Rational]),
            Type::Real,
        ]);
        let target = Type::Sum(vec![Type::Integer, Type::Real]);
        conclusion.cast_strategies.insert(
            TypeToType {
                from: ty.clone(),
                to: target.clone(),
            },
            CastStrategy::SumToSum(
                [
                    (
                        Type::Product(vec![Type::Integer, Type::Rational]),
                        Type::Integer,
                    ),
                    (Type::Real, Type::Real),
                ]
                .into_iter()
                .collect(),
            ),
        );
        conclusion.cast_strategies.insert(
            TypeToType {
                from: Type::Product(vec![Type::Integer, Type::Rational]),
                to: Type::Integer,
            },
            CastStrategy::ProductToInner(Type::Integer),
        );
        let ret = EvalCfg::cast(Arc::new(conclusion), &value, &ty, &target);
        assert_eq!(
            ret,
            Value::Variant {
                ty: Type::Integer,
                value: Box::new(Value::Int(1))
            }
        );
    }

    #[test]
    fn test_product_to_product_recursion() {
        let mut conclusion = TypeConclusions::default();
        let value = Value::Product(
            [
                (Type::Integer, Value::Int(1)),
                (
                    Type::Product(vec![Type::Integer, Type::Rational]),
                    Value::Product(
                        [
                            (Type::Integer, Value::Int(2)),
                            (Type::Rational, Value::Int(3)),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );
        let ty = Type::Product(vec![
            Type::Integer,
            Type::Product(vec![Type::Integer, Type::Rational]),
        ]);
        let target = Type::Product(vec![Type::Integer, Type::Rational]);
        conclusion.cast_strategies.insert(
            TypeToType {
                from: ty.clone(),
                to: target.clone(),
            },
            CastStrategy::ProductToProduct(
                [
                    (Type::Integer, Type::Integer),
                    (
                        Type::Product(vec![Type::Integer, Type::Rational]),
                        Type::Rational,
                    ),
                ]
                .into_iter()
                .collect(),
            ),
        );
        conclusion.cast_strategies.insert(
            TypeToType {
                from: Type::Product(vec![Type::Integer, Type::Rational]),
                to: Type::Rational,
            },
            CastStrategy::ProductToInner(Type::Rational),
        );
        let ret = EvalCfg::cast(Arc::new(conclusion), &value, &ty, &target);
        assert_eq!(
            ret,
            Value::Product(
                [
                    (Type::Integer, Value::Int(1)),
                    (Type::Rational, Value::Int(3)),
                ]
                .into_iter()
                .collect()
            )
        );
    }

    #[test]
    fn test_inner_to_sum_recursion() {
        let mut conclusion = TypeConclusions::default();
        let value = Value::Product(
            [
                (Type::Integer, Value::Int(1)),
                (Type::Rational, Value::Int(2)),
            ]
            .into_iter()
            .collect(),
        );
        let ty = Type::Product(vec![Type::Integer, Type::Rational]);
        let target = Type::Sum(vec![Type::Integer, Type::Rational]);
        conclusion.cast_strategies.insert(
            TypeToType {
                from: ty.clone(),
                to: target.clone(),
            },
            CastStrategy::InnerToSum(Type::Integer),
        );
        conclusion.cast_strategies.insert(
            TypeToType {
                from: Type::Product(vec![Type::Integer, Type::Rational]),
                to: Type::Integer,
            },
            CastStrategy::ProductToInner(Type::Integer),
        );
        let ret = EvalCfg::cast(Arc::new(conclusion), &value, &ty, &target);
        assert_eq!(
            ret,
            Value::Variant {
                ty: Type::Integer,
                value: Box::new(Value::Int(1))
            }
        );
    }

    #[test]
    fn test_product_to_inner() {
        let mut conclusion = TypeConclusions::default();
        let value = Value::Product(
            [
                (Type::Integer, Value::Int(1)),
                (
                    Type::Product(vec![Type::Integer, Type::Rational]),
                    Value::Product(
                        [
                            (Type::Integer, Value::Int(2)),
                            (Type::Rational, Value::Int(3)),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );
        let ty = Type::Product(vec![
            Type::Integer,
            Type::Product(vec![Type::Integer, Type::Rational]),
        ]);
        let target = Type::Rational;
        conclusion.cast_strategies.insert(
            TypeToType {
                from: ty.clone(),
                to: target.clone(),
            },
            CastStrategy::ProductToInner(Type::Product(vec![Type::Integer, Type::Rational])),
        );
        conclusion.cast_strategies.insert(
            TypeToType {
                from: Type::Product(vec![Type::Integer, Type::Rational]),
                to: Type::Rational,
            },
            CastStrategy::ProductToInner(Type::Rational),
        );
        let ret = EvalCfg::cast(Arc::new(conclusion), &value, &ty, &target);
        assert_eq!(ret, Value::Int(3));
    }
}
