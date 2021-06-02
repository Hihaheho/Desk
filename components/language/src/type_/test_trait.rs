use super::*;

#[test]
fn is_subtype_of_returns_false() {
    assert!(!Trait::new(vec![Arrow::new(Type::Number, Type::String)])
        .is_subtype_of(&Trait::new(vec![Arrow::new(Type::String, Type::Number)])));
}

#[test]
fn is_subtype_of_returns_true_if_superset() {
    assert!(Trait::new(vec![
        Arrow::new(Type::Number, Type::Number),
        Arrow::new(Type::Number, Type::String)
    ])
    .is_subtype_of(&Trait::new(vec![Arrow::new(Type::Number, Type::Number)])));
}

#[test]
fn is_subtype_of_returns_true_if_superset_of_subtypes() {
    assert!(Trait::new(vec![Arrow::new(
        Type::Number,
        Type::product(vec![Type::String, Type::Number])
    )])
    .is_subtype_of(&Trait::new(vec![Arrow::new(Type::Number, Type::Number),])));
}
