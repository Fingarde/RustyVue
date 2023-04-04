use serde::Deserialize;

use crate::error::Error;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    #[serde(default = "default::port")]
    pub port: u16,
    #[serde(default = "default::address")]
    pub address: String,
}

mod default {
    pub fn port() -> u16 {
        8080
    }

    pub fn address() -> String {
        "127.0.0.1".to_string()
    }
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, Error> {
        envy::prefixed("SERVER_")
            .from_env::<Self>()
            .map_err(Into::into)
    }
}
