#[macro_export]
macro_rules! unwrap_and_log {
    ( $( $x:expr ),* ) => {{
        use protocol::futures::future::ready;
        use tracing::error;
        |item| {
            ready(match item {
                Ok(ok) => Some(ok),
                Err(err) => {
                    error!("{}", err);
                    None
                }
            })
        }
    }};
}
