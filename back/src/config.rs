use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    #[serde(default = "server::default_port")]
    pub port: u16,
    #[serde(default = "server::default_address")]
    pub address: String,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    #[serde(default = "database::default_port")]
    pub port: u16,
    #[serde(default = "database::default_address")]
    pub address: String,
    #[serde(default = "database::default_username")]
    pub username: String,
    #[serde(default = "database::default_password")]
    pub password: String,
    #[serde(default = "database::default_database")]
    pub database: String,
}

mod database {
    pub fn default_port() -> u16 {
        5432
    }

    pub fn default_address() -> String {
        "127.0.0.1".to_string()
    }

    pub fn default_username() -> String {
        "postgres".to_string()
    }

    pub fn default_password() -> String {
        "postgres".to_string()
    }

    pub fn default_database() -> String {
        "postgres".to_string()
    }
}

mod server {
    pub fn default_port() -> u16 {
        8080
    }

    pub fn default_address() -> String {
        "127.0.0.1".to_string()
    }
}
