use log::{debug, warn};

pub fn load_env() {
    // load .env file and initialize logger
    let dotenv = dotenvy::dotenv();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // if .env file is not found, warn the user
    if dotenv.is_err() {
        warn!("No .env file found");
    }

    // print all env vars in debug mode
    for (key, value) in std::env::vars() {
        debug!("{}: {}", key, value);
    }
}
