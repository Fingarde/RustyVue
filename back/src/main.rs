use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

use diesel::prelude::*;

use crate::config::database::DatabaseConfig;
use crate::config::server::ServerConfig;
use log::info;

mod auth;
mod config;
mod controller;
mod database;
mod error;
mod model;
mod router;
mod schema;
mod utils;

use crate::error::Error;
use crate::model::Post;
use crate::router::RouterFactory;
use crate::router::{auth::AuthRouterFactory, post::PostRouterFactory};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // load env + initialize logger
    utils::load_env();

    // read configs from env vars
    let server_config = ServerConfig::from_env()?;
    let database_config = DatabaseConfig::from_env()?;

    // create database connection pool
    let pool = database::connect(database_config);

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
async fn index(json: web::Json<Post>, pool: web::Data<database::Pool>) -> impl Responder {
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
async fn list_posts(pool: web::Data<database::Pool>) -> HttpResponse {
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
