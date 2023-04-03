use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    Actix(#[from] actix_web::Error),
    #[error("data store disconnected")]
    Io(#[from] std::io::Error),
    #[error("data store disconnected")]
    Envy(#[from] envy::Error),
}