use std::{fmt::Display, u64};

use serde::{de, Deserialize};

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

#[derive(Deserialize, Debug)]
pub struct AuthorizationInfo {
    pub application: Application,
    pub user: User,
}

#[derive(Deserialize, Debug)]
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
