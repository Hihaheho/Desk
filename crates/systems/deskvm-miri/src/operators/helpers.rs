use crate::value::Value;

pub fn lr(value: &Value) -> (&Value, &Value) {
    let Value::Product(values) = value else { panic!("Expected product for integer operation")};
    let vec: Vec<_> = values
        .iter()
        .map(|(key, value)| match key {
            deskc_type::Type::Label { label, item: _ } => {
                let left = *label == "l".into();
                (left, value)
            }
            _ => (false, value),
        })
        .collect();
    let l = vec
        .iter()
        .find(|(left, _)| *left)
        .expect("left operand of operator not found")
        .1;
    let r = vec
        .iter()
        .find(|(left, _)| !*left)
        .expect("right operand of operator not found")
        .1;
    (l, r)
}
