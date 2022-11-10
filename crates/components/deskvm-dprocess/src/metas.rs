use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

#[derive(Debug, Default, Clone)]
/// This is used in Process and Processor for storing any kind of data especially for scheduling.
pub struct Metas {
    metas: HashMap<TypeId, Arc<dyn Any>>,
}

impl Metas {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: Any>(&mut self, value: T) {
        self.metas.insert(TypeId::of::<T>(), Arc::new(value));
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        self.metas
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    pub fn remove<T: Any>(&mut self) {
        self.metas.remove(&TypeId::of::<T>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metas() {
        let mut metas = Metas::new();
        metas.insert(1);
        metas.insert(2);
        metas.insert(3);
        assert_eq!(metas.get::<i32>(), Some(&3));
        metas.remove::<i32>();
        assert_eq!(metas.get::<i32>(), None);
    }
}
