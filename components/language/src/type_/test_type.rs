use super::*;

#[test]
fn product() {
    assert_eq!(
        Type::product(vec![Type::Number, Type::String, Type::Bool]),
        Type::Product(Set::new(vec![Type::Number, Type::String, Type::Bool]))
    );
}

#[test]
fn sum() {
    assert_eq!(
        Type::sum(vec![Type::Number, Type::String, Type::Bool]),
        Type::Sum(Set::new(vec![Type::Number, Type::String, Type::Bool]))
    );
}

#[test]
fn does_not_simplifies() {
    assert_eq!(Type::Number.remove_verbose_composite_type(), &Type::Number);
}

#[test]
fn does_not_simplifies_product() {
    assert_eq!(
        Type::product(vec![Type::Number, Type::String]).remove_verbose_composite_type(),
        &Type::product(vec![Type::Number, Type::String])
    )
}

#[test]
fn does_not_simplifies_sum() {
    assert_eq!(
        Type::sum(vec![Type::Number, Type::String]).remove_verbose_composite_type(),
        &Type::sum(vec![Type::Number, Type::String])
    )
}

#[test]
fn simplifies_single_item_product() {
    assert_eq!(
        Type::product(vec![Type::Number]).remove_verbose_composite_type(),
        &Type::Number
    )
}

#[test]
fn simplifies_single_item_sum() {
    assert_eq!(
        Type::sum(vec![Type::Number]).remove_verbose_composite_type(),
        &Type::Number
    )
}

#[test]
fn simplifies_into_unit() {
    assert_eq!(
        Type::sum(vec![]).remove_verbose_composite_type(),
        &Type::Unit
    );
    assert_eq!(
        Type::product(vec![]).remove_verbose_composite_type(),
        &Type::Unit
    );
}

#[test]
fn is_subtype_of_returns_true_if_unit() {
    assert!(Type::Unit.is_subtype_of(&Type::Unit));
}

#[test]
fn is_subtype_of_returns_false_if_not_equals() {
    assert!(!Type::Unit.is_subtype_of(&Type::Bool));
}

#[test]
fn is_subtype_of_returns_true_if_bool() {
    assert!(Type::Bool.is_subtype_of(&Type::Bool));
}

#[test]
fn is_subtype_of_returns_true_if_string() {
    assert!(Type::String.is_subtype_of(&Type::String));
}

#[test]
fn is_subtype_of_returns_true_if_number() {
    assert!(Type::Number.is_subtype_of(&Type::Number));
}

#[test]
fn is_subtype_of_returns_true_if_label_is_equal() {
    assert!(Type::Label("".to_string()).is_subtype_of(&Type::Label("".to_string())));
}

#[test]
fn is_subtype_of_returns_false_if_label_is_not_equal() {
    assert!(!Type::Label("a".to_string()).is_subtype_of(&Type::Label("b".to_string())));
}

#[test]
fn is_subtype_of_returns_true_if_product_is_superset() {
    assert!(
        Type::Product(Set::new(vec![Type::Number, Type::String, Type::Bool]))
            .is_subtype_of(&Type::Product(Set::new(vec![Type::Number, Type::Bool])))
    );
}

#[test]
fn is_subtype_of_returns_false_if_product_is_not_superset() {
    assert!(!Type::Product(Set::new(vec![
        Type::Number,
        Type::Product(Set::new(vec![Type::Number, Type::Bool])),
    ]))
    .is_subtype_of(&Type::Product(Set::new(vec![
        Type::Number,
        Type::Product(Set::new(vec![Type::Number, Type::String, Type::Bool])),
    ]))));
}

#[test]
fn is_subtype_of_returns_true_if_all_elements_of_product_are_subtype() {
    assert!(Type::Product(Set::new(vec![
        Type::Number,
        Type::Product(Set::new(vec![Type::Number, Type::String, Type::Bool])),
    ]))
    .is_subtype_of(&Type::Product(Set::new(vec![
        Type::Number,
        Type::Product(Set::new(vec![Type::Number, Type::Bool])),
    ]))));
}

#[test]
fn is_subtype_of_returns_true_if_sum_is_subset() {
    assert!(
        Type::Sum(Set::new(vec![Type::Number, Type::Bool])).is_subtype_of(&Type::Sum(Set::new(
            vec![Type::Number, Type::String, Type::Bool]
        )))
    );
}

#[test]
fn is_subtype_of_returns_false_if_sum_is_not_subset() {
    assert!(!Type::Sum(Set::new(vec![
        Type::Number,
        Type::Product(Set::new(vec![Type::Number])),
    ]))
    .is_subtype_of(&Type::Sum(Set::new(vec![
        Type::Number,
        Type::Product(Set::new(vec![Type::Number, Type::String])),
    ]))));
}

#[test]
fn is_subtype_of_returns_true_if_all_elements_of_sum_are_subtype() {
    assert!(Type::Sum(Set::new(vec![
        Type::Number,
        Type::Product(Set::new(vec![Type::Number, Type::String, Type::Bool])),
    ]))
    .is_subtype_of(&Type::Sum(Set::new(vec![
        Type::Number,
        Type::Product(Set::new(vec![Type::Number, Type::Bool])),
    ]))));
}

#[test]
fn is_subtype_of_returns_true_if_product_contains_subtype() {
    assert!(Type::Product(Set::new(vec![Type::Number, Type::Bool])).is_subtype_of(&Type::Number));

    assert!(Type::Product(Set::new(vec![
        Type::Product(Set::new(vec![Type::Number, Type::String, Type::Bool])),
        Type::Number,
    ]))
    .is_subtype_of(&Type::Product(Set::new(vec![Type::Number, Type::Bool]))));
}

#[test]
fn is_subtype_of_returns_true_if_supertype_is_included_in_sum() {
    assert!(Type::Number.is_subtype_of(&Type::Sum(Set::new(vec![Type::Number, Type::Bool]))));
    assert!(
        Type::Sum(Set::new(vec![Type::Number, Type::Bool])).is_subtype_of(&Type::Sum(Set::new(
            vec![
                Type::Sum(Set::new(vec![Type::Number, Type::String, Type::Bool])),
                Type::Number,
            ]
        )))
    );

    assert!(
        Type::Sum(Set::new(vec![Type::Number, Type::Bool])).is_subtype_of(&Type::Sum(Set::new(
            vec![
                Type::Sum(Set::new(vec![Type::Number, Type::String, Type::Bool])),
                Type::Number,
            ]
        )))
    );
}

#[test]
fn is_subtype_of_simplifies() {
    assert!(Type::product(vec![Type::Number]).is_subtype_of(&Type::sum(vec![Type::Number])));
    assert!(Type::sum(vec![Type::Number]).is_subtype_of(&Type::product(vec![Type::Number])));
}

#[test]
fn is_subtype_of_returns_false_if_array_item_is_different() {
    assert!(!Type::Array(Box::new(Type::String)).is_subtype_of(&Type::Array(Box::new(Type::Number))))
}

#[test]
fn is_subtype_of_returns_true_if_array_item_is_subtype() {
    assert!(
        Type::Array(Box::new(Type::product(vec![Type::Number, Type::String])))
            .is_subtype_of(&Type::Array(Box::new(Type::Number)))
    )
}

#[test]
fn is_subtype_of_returns_true_if_function_is_subtype() {
    assert!(Type::Function(Arrow::new(
        Type::Number,
        Type::product(vec![Type::Number, Type::Bool]),
    ))
    .is_subtype_of(&Type::Function(Arrow::new(Type::Number, Type::Bool))));
}

#[test]
fn is_subtype_of_returns_false_if_function_is_not_subtype() {
    assert!(!Type::Function(Arrow::new(
        Type::product(vec![Type::Number, Type::Bool]),
        Type::Number,
    ))
    .is_subtype_of(&Type::Function(Arrow::new(Type::Number, Type::Bool))));
}

#[test]
fn is_subtype_of_returns_true_if_trait_is_subtype() {
    assert!(Trait::new(vec![
        Arrow::new(Type::sum(vec![Type::String, Type::Number]), Type::Bool),
        Arrow::new(Type::Number, Type::String)
    ])
    .is_subtype_of(&Trait::new(vec![Arrow::new(Type::Number, Type::Bool)])));
}

#[test]
fn is_subtype_of_returns_false_if_trait_is_not_subtype() {
    assert!(!Trait::new(vec![Arrow::new(Type::Number, Type::Bool)])
        .is_subtype_of(&Trait::new(vec![Arrow::new(Type::Bool, Type::Number)])));
}