use actix_web::{
    dev::ServiceRequest,
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use log::{debug, info, warn};

pub mod auth;
mod config;
pub mod controller;
mod error;
mod model;
mod router;
mod schema;

use crate::config::{DatabaseConfig, ServerConfig};
use crate::error::Error;
use crate::model::Post;
use crate::router::RouterFactory;
use crate::router::{auth::AuthRouterFactory, post::PostRouterFactory};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> Result<(), Error> {
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

    // read configs from env vars
    let server_config = envy::prefixed("SERVER_").from_env::<ServerConfig>()?;
    let database_config = envy::prefixed("DATABASE_").from_env::<DatabaseConfig>()?;

    // create postgres connection string
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database_config.username,
        database_config.password,
        database_config.address,
        database_config.port,
        database_config.database
    );

    info!("Connecting to database at {}", database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    info!(
        "Starting server on http://{}:{}/",
        server_config.address, server_config.port
    );

    HttpServer::new(move || {
        App::new()
            // add database connection to app data
            .app_data(Data::new(pool.clone()))
            // enable logger - always register actix-web Logger middleware last because it is called in reverse order
            .wrap(actix_web::middleware::Logger::default())
            .configure(AuthRouterFactory::config)
            .configure(PostRouterFactory::config)
    })
    .bind((server_config.address, server_config.port))?
    .run()
    .await?;

    info!("Server stopped");

    Ok(())
}

#[post("/")]
async fn index(json: web::Json<Post>, pool: web::Data<Pool>) -> impl Responder {
    use crate::schema::posts::dsl::*;

    let mut conn = pool.get().unwrap();

    let form = json.into_inner();

    let result = diesel::insert_into(posts)
        .values(form)
        .get_result::<Post>(&mut conn)
        .expect("Error saving new post");

    format!("Inserted {:?}", result)
}

// list all
#[get("/posts")]
async fn list_posts(pool: web::Data<Pool>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let mut conn = pool.get().unwrap();

    let results = posts
        .limit(5)
        .filter(published.eq(true))
        .load::<Post>(&mut conn)
        .expect("Error loading posts");

    HttpResponse::Ok().json(results)
}

#[get("/login")]
async fn login() -> impl Responder {
    "Login"
}

#[get("/register")]
async fn register() -> impl Responder {
    "Register"
}
