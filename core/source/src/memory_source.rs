use card::{Source, Value};

pub struct MemorySource<T> {
    value: T,
}

impl<T: Value> MemorySource<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> Source for MemorySource<T> {}
