use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use log::info;
use once_cell::sync::OnceCell;

use crate::config::database::DatabaseConfig;

pub mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub static DATABASE_POOL: OnceCell<Pool> = OnceCell::new();

fn connect(config: DatabaseConfig) -> Pool {
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

pub fn init(config: DatabaseConfig) {
    let pool = connect(config);
    DATABASE_POOL.set(pool).expect("Failed to set database pool.");
}
