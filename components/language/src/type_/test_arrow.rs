use super::*;

#[test]
fn is_subtype_of_returns_false_if_input_is_not_supertype() {
    assert!(
        !Arrow::new(Type::product(vec![Type::Number, Type::Bool]), Type::Bool)
            .is_subtype_of(&Arrow::new(Type::Number, Type::Bool))
    );
}

#[test]
fn is_subtype_of_returns_true_if_input_is_supertype() {
    assert!(
        Arrow::new(Type::sum(vec![Type::Number, Type::Bool]), Type::Bool)
            .is_subtype_of(&Arrow::new(Type::Number, Type::Bool))
    );
}

#[test]
fn is_subtype_of_returns_false_if_output_is_not_subtype() {
    assert!(
        !Arrow::new(Type::Number, Type::sum(vec![Type::Number, Type::Bool]))
            .is_subtype_of(&Arrow::new(Type::Number, Type::Bool))
    );
}

#[test]
fn is_subtype_of_returns_true_if_output_is_subtype() {
    assert!(
        Arrow::new(Type::Number, Type::product(vec![Type::Number, Type::Bool]))
            .is_subtype_of(&Arrow::new(Type::Number, Type::Bool))
    );
}
