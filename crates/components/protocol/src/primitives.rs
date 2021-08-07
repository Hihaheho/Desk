use derive_more::{From, Into};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, From)]
pub struct RoomName(pub String);

#[derive(Serialize, Deserialize, Debug, From)]
pub struct RoomId(pub Uuid);
#[derive(Serialize, Deserialize, Debug, From)]
pub struct RoomLocalUserId(pub Uuid);
#[derive(Serialize, Deserialize, Debug)]

pub struct UserId(pub Uuid);

#[derive(Serialize, Deserialize, Debug, Into)]

pub struct Topic {
    pub crate_name: String,
    pub topic_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bytes(#[serde(with = "serde_bytes")] pub Vec<u8>);

impl<T: Into<Vec<u8>>> From<T> for Bytes {
    fn from(from: T) -> Self {
        Self(from.into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OneTimeCode(String);

impl<T: Into<String>> From<T> for OneTimeCode {
    fn from(from: T) -> Self {
        Self(from.into())
    }
}
