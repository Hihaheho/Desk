use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "c")] // stands for code
pub enum EntranceCommand {
    /// Ask to join
    AskJoinAnonymous { room_id: RoomId },
    /// Ask to join with user_id
    AskJoin { room_id: RoomId },
    /// Cannot accept an anonymous user except in a public room
    Accept {
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
    },
    AcceptAnonymous {
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
        one_time_code: OneTimeCode,
    },
    Reject {
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
    },
}
