use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use log::info;

use crate::config::database::DatabaseConfig;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn connect(config: DatabaseConfig) -> Pool {
    // create postgres connection string
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.address, config.port, config.database
    );

    info!("Connecting to database at {}", database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
