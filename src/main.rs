mod dynamodb;
mod common;
mod routes;
mod tracing;
mod config;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::http::header;
use std::path::PathBuf;
use std::sync::OnceLock;
use actix_web::web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = config::Config.clone();

    if let Err(_) = tracing::init_tracing() {
        eprintln!("Failed to setup tracing");
        panic!("Failed to initialize tracing");
    }

    let ddb_table = std::env::var("DYNAMODB_TABLE");

    if ddb_table.is_err() {
        tracing::error!(
            task = "Environment variable check",
            result = "failure",
            "DYNAMODB_TABLE environment variable is not set"
        );
        panic!("DYNAMODB_TABLE environment variable is required");
    }

    let ddb_app_state = dynamodb::initialize_dynamodb(ddb_table.unwrap().as_str()).await;

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(ddb_app_state.clone()))
            .configure(routes::config)
    })
    .bind((config::Config.http_server.host.clone(), config::Config.http_server.port))?;

    tracing::info!(
        task = "Actix setup",
        result = "success",
        "Actix web server successfully configured"
    );

    let port = config::Config.http_server.port;

    tracing::info!(
        task = "Actix setup",
        port = port,
        "Actix web server running on {}:{}",
        config::Config.http_server.host,
        port
    );

    server.run().await
}
