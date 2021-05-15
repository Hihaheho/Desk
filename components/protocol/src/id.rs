pub type Id = uuid::Uuid;
pub fn create_new_id() -> Id {
    uuid::Uuid::new_v4()
}

pub fn create_consistent_id(name: String) -> Id {
    uuid::Uuid::new_v5(
        &uuid::Uuid::nil(),
        format!("{}/{}", std::env::var("CARGO_PKG_NAME").unwrap(), name).as_bytes(),
    )
}

#[macro_export]
macro_rules! consistent_id {
    () => {
        uuid::Uuid::new_v5();
    };
}
