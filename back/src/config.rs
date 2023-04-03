use serde::{Deserialize};

fn default_port() -> u16 {
    8080
}

fn default_address() -> String {
    "127.0.0.1".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_address")]
    pub address: String
}