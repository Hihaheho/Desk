use crate::AuthenticationErrorCode;

use super::primitives::*;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "c")] // stands for code
pub enum Event {
    LoggedIn {
        user_id: UserId,
    },
    /// Created the room
    CreatedRoom {
        room_name: RoomName,
        room_id: RoomId,
    },
    /// Needed when joining private room as an anonymous user
    OneTimeCode {
        room_id: RoomId,
        one_time_code: OneTimeCode,
    },
    /// Accepted to join
    Accepted {
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
        room_owner_local_user_id: RoomLocalUserId,
    },
    /// Rejected to join
    Rejected {
        room_id: RoomId,
    },
    /// Asked to join
    AskedJoin {
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
        user_id: UserId,
    },
    AskedJoinAnonymous {
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
    },
    TopicUpdate {
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
        topic: Topic,
    },
    TopicPush {
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
        topic: Topic,
    },
    Error {
        code: ErrorCode,
        message: String,
    },
    #[serde(other)]
    Unknown,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ErrorCode {
    Authentication(AuthenticationErrorCode),
    UnexpectedOperation,
    InternalError,
    RoomNotFound,
    UnknownOperation,
}
