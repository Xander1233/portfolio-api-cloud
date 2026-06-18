use std::str::FromStr;
use std::time::Instant;

use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::Error;
use tracing::Instrument;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter, Layer};

use crate::config::{ConfigStruct, CONFIG};

pub fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    let config: &ConfigStruct = &CONFIG;

    let file_appender =
        tracing_appender::rolling::hourly(config.logging.directory.clone(), config.logging.file.clone());
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.logging.level.clone()));

    let env_filter_level = match env_filter.max_level_hint() {
        Some(t) => t.to_string(),
        None => "UNKNOWN".to_string(),
    };

    let stdout_filter_level =
        tracing_subscriber::filter::LevelFilter::from_str(&config.logging.level)
            .unwrap_or(tracing_subscriber::filter::LevelFilter::INFO);

    let stdout_layer = fmt::layer()
        .pretty()
        .with_target(false)
        .with_filter(stdout_filter_level);

    let file_layer = fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true)
        .with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    tracing::info!(
        level = %config.logging.level,
        global_level = %env_filter_level,
        directory = %config.logging.directory,
        file = %config.logging.file,
        "tracing initialized"
    );

    guard
}

/// Per-request middleware: emits an `http_request` tracing span with method,
/// path, status, and elapsed time. Use via `.wrap(actix_web::middleware::from_fn(log_request))`.
pub async fn log_request<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, Error>
where
    B: MessageBody,
{
    let started = Instant::now();
    let method = req.method().clone();
    let path = req.path().to_owned();

    let span = tracing::info_span!(
        "http_request",
        method = %method,
        path = %path,
        status = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    );

    async move {
        let response = next.call(req).await?;
        let status = response.status().as_u16();
        let duration_ms = started.elapsed().as_millis() as u64;

        tracing::Span::current().record("status", status);
        tracing::Span::current().record("duration_ms", duration_ms);

        if response.status().is_server_error() {
            tracing::warn!(status, duration_ms, "request completed with server error");
        } else if response.status().is_client_error() {
            tracing::info!(status, duration_ms, "request rejected");
        } else {
            tracing::info!(status, duration_ms, "request completed");
        }

        Ok(response)
    }
    .instrument(span)
    .await
}
