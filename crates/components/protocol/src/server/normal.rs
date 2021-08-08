use crate::primitives::*;
use crate::{ServerContext, ServerInput, ServerStateSet};

use super::ServerState;
use async_trait::async_trait;

#[derive(Debug, PartialEq, Clone)]
pub struct Normal {
    pub user_id: UserId,
}

#[async_trait]
impl ServerState for Normal {
    async fn handle<T: Send + Sync>(
        &self,
        _context: &mut ServerContext<T>,
        _input: &ServerInput,
    ) -> ServerStateSet {
        todo!()
    }
}
