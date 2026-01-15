use std::str::FromStr;
use config::ValueKind::String;
use tracing::log::LevelFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::prelude::*;
use crate::config::{Config, ConfigStruct};

pub fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {

    let config: &ConfigStruct = &Config;

    let file_appender =
        tracing_appender::rolling::hourly(config.logging.directory.clone(), config.logging.file.clone());
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.logging.level.clone()));

    let env_filter_level = match env_filter.max_level_hint() {
        Some(t) => t.to_string(),
        None => "UNKNOWN".to_string(),
    };

    let stdout_filter_level = tracing_subscriber::filter::LevelFilter::from_str(&config.logging.level)
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
        .with(env_filter)     // global filter
        .with(stdout_layer)   // stdout
        .with(file_layer)     // file
        .init();

    tracing::info!(
        task = "Initialize Tracing",
        result = "success",
        "Tracing initialized with level: {}; Global level: {}",
        config.logging.level,
        env_filter_level
    );

    guard
}