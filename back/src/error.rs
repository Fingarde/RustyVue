use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error("Actix error")]
    Actix(#[from] actix_web::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Envy error")]
    Envy(#[from] envy::Error),
    #[error("Diesel error")]
    Diesel(#[from] diesel::ConnectionError),
}