mod dynamodb;
mod common;
mod routes;
mod logging;
mod config;
mod queries;

use std::time::Duration;
use actix_web::web;
use actix_cors::Cors;
use actix_web::http::header;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = (*config::Config).clone();

    if let Err(_) = logging::init_tracing() {
        eprintln!("Failed to setup tracing");
        panic!("Failed to initialize tracing");
    }

    let ddb_table = std::env::var("DYNAMODB_TABLE").unwrap_or_else(|_| {
        tracing::error!(task = "env", result = "missing", "DYNAMODB_TABLE is not set");
        std::process::exit(1);
    });

    let ddb_app_state = dynamodb::initialize_dynamodb(&ddb_table, Duration::new(config.http_server.cache_ttl_seconds, 0)).await;

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(Cors::permissive()
                .allowed_origin_fn(|origin, _req_head| {
                    origin.as_bytes().starts_with(b"http://localhost")
                        || origin.as_bytes().starts_with(b"http://127.0.0.1")
                        || origin.as_bytes().starts_with(b"https://david-neidhart.de")
                        || origin.as_bytes().starts_with(b"https://cdn.david-neidhart.de")
                })
                .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
                .allowed_headers(vec![
                    header::ACCEPT,
                    header::AUTHORIZATION,
                    header::CONTENT_TYPE,
                    header::HeaderName::from_static("x-requested-with"),
                    header::HeaderName::from_static("x-xsrf-token"),
                ])
                .supports_credentials()
                .max_age(3600))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(ddb_app_state.clone()))
            .configure(routes::config)
    })
    .bind(((*config::Config).http_server.host.clone(), (*config::Config).http_server.port))?;

    tracing::info!(
        task = "Actix setup",
        result = "success",
        "Actix web server successfully configured"
    );

    let port = (*config::Config).http_server.port;

    tracing::info!(
        task = "Actix setup",
        port = port,
        "Actix web server running on {}:{}",
        (*config::Config).http_server.host,
        port
    );

    server.run().await
}
