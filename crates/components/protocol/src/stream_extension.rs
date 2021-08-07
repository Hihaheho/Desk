use std::fmt::Display;

use futures::future::{ready, Ready};
use tracing::error;

pub fn unwrap_and_log<T, E: Display>(item: Result<T, E>) -> Ready<Option<T>> {
    match item {
        Ok(ok) => ready(Some(ok)),
        Err(err) => {
            error!("{}", err);
            ready(None)
        }
    }
}
