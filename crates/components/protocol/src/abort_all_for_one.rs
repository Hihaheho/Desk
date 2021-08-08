use futures::future::{join_all, select_all};
use tokio::task::{JoinError, JoinHandle};

pub async fn abort_all_for_one<T, I>(tasks: I) -> Result<T, JoinError>
where
    I: IntoIterator<Item = JoinHandle<T>>,
{
    let (result, _, tasks) = select_all(tasks.into_iter()).await;
    tasks.iter().for_each(|task| task.abort());
    let _ = join_all(tasks).await;
    result
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use futures::future::{pending, ready};

    use super::*;

    #[tokio::test]
    async fn returns_first_one() {
        let tasks = vec![tokio::spawn(ready(1u8)), tokio::spawn(ready(2u8))];
        assert!(matches!(abort_all_for_one(tasks).await, Ok(1u8)));
    }

    #[tokio::test]
    async fn skips_pending() {
        let tasks = vec![tokio::spawn(pending()), tokio::spawn(ready(2u8))];
        assert!(matches!(abort_all_for_one(tasks).await, Ok(2u8)));
    }

    #[tokio::test]
    async fn aborts_all() {
        let arc = Arc::new(1u8);
        let moved = arc.clone();
        let tasks = vec![
            tokio::spawn(ready(())),
            tokio::spawn(async move {
                let () = pending().await;
                println!("{}", moved);
            }),
        ];
        assert_eq!(Arc::<u8>::strong_count(&arc), 2);
        assert!(abort_all_for_one(tasks).await.is_ok());
        assert_eq!(Arc::<u8>::strong_count(&arc), 1);
    }
}
