use std::{fmt::Display, u64};

use serde::{de, Deserialize, Serialize};

/// Discord specific identifier
/// For more information view [Snowflakes](https://discord.com/developers/docs/reference#snowflakes)
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Snowflake(u64);

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val: &str = Deserialize::deserialize(deserializer)?;
        Ok(Snowflake(val.parse().map_err(de::Error::custom)?))
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl From<u64> for Snowflake {
    fn from(value: u64) -> Self {
        Snowflake(value)
    }
}

#[derive(Deserialize, Debug)]
pub struct AuthorizationInfo {
    pub application: Application,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub avatar: Option<String>,
    pub global_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Application {
    pub id: Snowflake,
    pub name: String,
}
