use serde::Deserialize;

use crate::error::Error;

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    #[serde(default = "default::port")]
    pub port: u16,
    #[serde(default = "default::address")]
    pub address: String,
    #[serde(default = "default::username")]
    pub username: String,
    #[serde(default = "default::password")]
    pub password: String,
    #[serde(default = "default::database")]
    pub database: String,
}

mod default {
    pub fn port() -> u16 {
        5432
    }

    pub fn address() -> String {
        "127.0.0.1".to_string()
    }

    pub fn username() -> String {
        "postgres".to_string()
    }

    pub fn password() -> String {
        "postgres".to_string()
    }

    pub fn database() -> String {
        "postgres".to_string()
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, Error> {
        envy::prefixed("DATABASE_").from_env::<Self>().map_err(Into::into)
    }
}