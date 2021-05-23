use super::*;

#[test]
fn is_subtype_of_returns_false() {
    assert!(!Trait::new(vec![Arrow::new(Type::Number, Type::Bool)])
        .is_subtype_of(&Trait::new(vec![Arrow::new(Type::Bool, Type::Number)])));
}

#[test]
fn is_subtype_of_returns_true_if_superset() {
    assert!(Trait::new(vec![
        Arrow::new(Type::Number, Type::Bool),
        Arrow::new(Type::Number, Type::String)
    ])
    .is_subtype_of(&Trait::new(vec![Arrow::new(Type::Number, Type::Bool)])));
}

#[test]
fn is_subtype_of_returns_true_if_superset_of_subtypes() {
    assert!(Trait::new(vec![Arrow::new(
        Type::Number,
        Type::product(vec![Type::String, Type::Bool])
    )])
    .is_subtype_of(&Trait::new(vec![Arrow::new(Type::Number, Type::Bool),])));
}
