use std::sync::Arc;

pub type QueryResult<T> = Result<Arc<T>, QueryError>;

#[derive(Debug, Clone)]
pub struct QueryError(pub Arc<Box<dyn std::error::Error + Send + Sync + 'static>>);

impl PartialEq for QueryError {
    fn eq(&self, _other: &Self) -> bool {
        // FIXME: this is not a good solution: we need Eq object safe
        // always returns false to occur recomputation always on error
        false
    }
}
impl Eq for QueryError {}

impl<T> From<T> for QueryError
where
    T: std::error::Error + Send + Sync + 'static,
{
    fn from(error: T) -> Self {
        QueryError(Arc::new(Box::new(error)))
    }
}
