use std::time::Duration;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use aws_config::SdkConfig;

use crate::config::{AwsSecrets, ConfigStruct};
use crate::routes;
use crate::state::AppState;

pub async fn run(
    config: ConfigStruct,
    secrets: AwsSecrets,
    shared_config: SdkConfig,
) -> std::io::Result<()> {
    let cache_ttl = Duration::from_secs(config.http_server.cache_ttl_seconds);
    let state = AppState::new(&shared_config, &secrets.ddb_table, cache_ttl);

    let host = config.http_server.host.clone();
    let port = config.http_server.port;
    let allowed_origins = config.http_server.allowed_origins.clone();
    let cdn_base_url = config.assets.cdn_base_url.clone();

    let state_data = web::Data::new(state);
    let config_data = web::Data::new(config);

    tracing::info!(
        host = %host,
        port,
        cache_ttl_secs = cache_ttl.as_secs(),
        cdn_base_url = %cdn_base_url,
        cors_origins = allowed_origins.len(),
        "actix web server starting"
    );

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::from_fn(crate::logging::log_request))
            .wrap(build_cors(&allowed_origins))
            .app_data(state_data.clone())
            .app_data(config_data.clone())
            .configure(routes::config)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

fn build_cors(allowed_origins: &[String]) -> Cors {
    let mut cors = Cors::default()
        .allowed_methods(["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
        .allowed_headers([
            header::ACCEPT,
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static("x-requested-with"),
            header::HeaderName::from_static("x-xsrf-token"),
        ])
        .max_age(3600);

    /*for origin in allowed_origins {
        cors = cors.allowed_origin(origin);
    }*/
    cors = cors.allow_any_origin();

    cors
}
