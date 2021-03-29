use card::{Card, Value};
use source::MemorySource;

pub fn create_memory_card<V: Value + 'static>(value: V) -> Card {
    let source = MemorySource::new(value);
    Card::new(source)
}
