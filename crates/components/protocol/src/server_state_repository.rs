use crate::{Bytes, Command, RoomId, RoomLocalUserId, Topic, UserId};
use futures::prelude::*;

#[non_exhaustive]
pub enum ServerStateRepositoryError {
    Other(String),
}

#[non_exhaustive]
pub enum RoomUpdate {
    InboundCommand { user_id: UserId, command: Command },
}

#[non_exhaustive]
pub enum TopicUpdate {
    Updated { all_contents: Vec<Bytes> },
    Added { content: Bytes },
}

trait ServerStateRepository {
    fn create_room(
        &mut self,
        room_id: RoomId,
    ) -> Result<Box<dyn Stream<Item = RoomUpdate>>, ServerStateRepositoryError>;

    fn send_command_to_room(
        &mut self,
        room_id: RoomId,
        sender: RoomLocalUserId,
        command: Command,
    ) -> Result<(), ServerStateRepositoryError>;

    // fn add_user_to_room(
    //     &mut self,
    //     room_id: RoomId,
    //     user_id: UserId,
    //     local_user_id: RoomLocalUserId,
    // ) -> Result<(), ServerStateRepositoryError>;

    // fn get_room_local_user_id(
    //     &mut self,
    //     room_id: RoomId,
    //     user_id: UserId,
    // ) -> Result<RoomLocalUserId, ServerStateRepositoryError>;

    fn update_topic(
        &mut self,
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
        topic: &Topic,
        all_contents: Vec<Bytes>,
    ) -> Result<(), ServerStateRepositoryError>;

    fn add_to_topic(
        &mut self,
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
        topic: &Topic,
        content: Bytes,
    ) -> Result<(), ServerStateRepositoryError>;

    fn listen_topic(
        &mut self,
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
        topic: &Topic,
    ) -> Result<Box<dyn Stream<Item = TopicUpdate>>, ServerStateRepositoryError>;
}
