use std::sync::Arc;

#[derive(Debug, Clone)]
/// `Arc<anyhow::Error>`
pub struct QueryError(pub Arc<anyhow::Error>);

impl PartialEq for QueryError {
    fn eq(&self, _other: &Self) -> bool {
        // FIXME: this is not a good solution: we need Eq object safe
        // always returns false to occur recomputation always on error
        false
    }
}
impl Eq for QueryError {}

impl<T: Into<anyhow::Error>> From<T> for QueryError {
    fn from(error: T) -> Self {
        QueryError(Arc::new(error.into()))
    }
}
