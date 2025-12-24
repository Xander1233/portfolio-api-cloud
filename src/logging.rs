use std::process;
use tracing::subscriber;
use tracing_subscriber::{fmt, Layer};
use tracing_subscriber::registry::Registry;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_loki;
use url::Url;
use crate::config::{Config, ConfigStruct};

pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {

    let config: &ConfigStruct = &*Config;

    let app_name = format!("{}-{}", config.general.hostname, process::id());

    let stdout_layer = fmt::layer()
        .pretty()
        .with_target(false)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    let file = std::fs::File::create(format!(
        "logs/{}-25.log",
        app_name
    ))?;
    let file_layer = fmt::layer().json().with_writer(file).with_filter(match &config.logging.level {
        level if level.eq_ignore_ascii_case("DEBUG") => tracing_subscriber::filter::LevelFilter::DEBUG,
        level if level.eq_ignore_ascii_case("INFO") => tracing_subscriber::filter::LevelFilter::INFO,
        level if level.eq_ignore_ascii_case("WARN") => tracing_subscriber::filter::LevelFilter::WARN,
        level if level.eq_ignore_ascii_case("ERROR") => tracing_subscriber::filter::LevelFilter::ERROR,
        level if level.eq_ignore_ascii_case("TRACE") => tracing_subscriber::filter::LevelFilter::TRACE,
        _ => tracing_subscriber::filter::LevelFilter::INFO,
    });

    let subscriber = tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer);

    subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber");

    tracing::info!(
        task = "tracing_setup",
        result = "success",
        "tracing successfully set up"
    );

    Ok(())
}