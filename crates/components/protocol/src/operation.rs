use super::primitives::*;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "c")] // stands for code
pub enum Operation {
    Login(Login),
    CreateRoom {
        room_name: RoomName,
    },
    CreatePublicRoom {
        room_name: RoomName,
    },
    /// Ask to join
    AskJoinAnonymous {
        room_id: RoomId,
    },
    /// Ask to join with user_id
    AskJoin {
        room_id: RoomId,
    },
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
    /// Update the topic with the contents
    Update {
        room_id: RoomId,
        topic: Topic,
        all_contents: Vec<Bytes>,
    },
    /// Push a content into the topic
    Push {
        room_id: RoomId,
        topic: Topic,
        contents: Bytes,
    },
    /// Listen to the topic
    Listen {
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
        topic: Topic,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize)] // No Debug to avoid expose a credential in debug log!
pub struct Login {
    #[serde(with = "serde_bytes")]
    token: Vec<u8>,
}

impl std::fmt::Debug for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Login").field("token", &"****").finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn debug_of_login_does_not_contain_credential() {
        let target = Operation::Login(Login {
            token: vec![1, 2, 3],
        });
        assert_eq!(format!("{:?}", target), r#"Login(Login { token: "****" })"#)
    }
}
