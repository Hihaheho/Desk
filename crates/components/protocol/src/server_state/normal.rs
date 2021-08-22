use std::collections::HashMap;

use crate::primitives::*;
use crate::{ServerContext, ServerInput, ServerStateDispatcher};

use super::CreatingRoom;
use super::ServerState;
use async_trait::async_trait;
use derive_new::new;

#[derive(Debug, PartialEq, Clone, new)]
pub struct Normal {
    pub user_id: UserId,
    #[new(default)]
    pub creating_rooms: HashMap<RoomName, CreatingRoom>,
}

#[async_trait]
impl ServerState for Normal {
    async fn handle(
        self,
        _context: &mut (impl ServerContext + Send + Sync),
        _input: &ServerInput,
    ) -> ServerStateDispatcher {
        todo!()
    }
}
