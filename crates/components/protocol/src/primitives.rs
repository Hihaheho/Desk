use derive_more::{From, Into};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RoomName(pub String);

impl<T: Into<String>> From<T> for RoomName {
    fn from(from: T) -> Self {
        Self(from.into())
    }
}

#[derive(Serialize, Deserialize, Debug, From, PartialEq, Clone)]
pub struct RoomId(pub Uuid);
#[derive(Serialize, Deserialize, Debug, From, PartialEq, Clone)]
pub struct RoomLocalUserId(pub Uuid);
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]

pub struct UserId(pub Uuid);

#[derive(Serialize, Deserialize, Debug, Into, PartialEq, Clone)]

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
