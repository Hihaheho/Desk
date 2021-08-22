use derive_more::{From, Into};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct RoomName(pub String);

impl<T: Into<String>> From<T> for RoomName {
    fn from(from: T) -> Self {
        Self(from.into())
    }
}

#[derive(Serialize, Deserialize, Debug, From, PartialEq, Eq, Hash, Clone)]
pub struct RoomId(pub Uuid);
#[derive(Serialize, Deserialize, Debug, From, PartialEq, Eq, Hash, Clone)]
pub struct RoomLocalUserId(pub Uuid);
impl RoomLocalUserId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize, Debug, From, PartialEq, Eq, Hash, Clone)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize, Debug, Into, PartialEq, Eq, Hash, Clone)]
pub struct Topic {
    pub crate_name: String,
    pub topic_name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Bytes(#[serde(with = "serde_bytes")] pub Vec<u8>);

impl<T: Into<Vec<u8>>> From<T> for Bytes {
    fn from(from: T) -> Self {
        Self(from.into())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Token {
    bytes: Bytes,
}

impl<T: Into<Bytes>> From<T> for Token {
    fn from(from: T) -> Self {
        Self { bytes: from.into() }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OneTimeCode(String);

impl<T: Into<String>> From<T> for OneTimeCode {
    fn from(from: T) -> Self {
        Self(from.into())
    }
}
