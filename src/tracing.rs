use std::fmt::Error;
use std::process;
use tracing::subscriber;
use tracing_subscriber::{fmt, Layer, Registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_loki;
use url::Url;
use crate::config::Config;

pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let app_name = format!("{}-{}", &Config.general.hostname, process::id());

    let stdout_layer = fmt::layer()
        .pretty()
        .with_target(false)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    let file = std::fs::File::create(format!(
        "logs/{}-25.log",
        app_name
    ))?;
    let file_layer = fmt::layer().json().with_writer(file).with_filter(match &Config.logging.level {
        level if level.eq_ignore_ascii_case("DEBUG") => tracing_subscriber::filter::LevelFilter::DEBUG,
        level if level.eq_ignore_ascii_case("INFO") => tracing_subscriber::filter::LevelFilter::INFO,
        level if level.eq_ignore_ascii_case("WARN") => tracing_subscriber::filter::LevelFilter::WARN,
        level if level.eq_ignore_ascii_case("ERROR") => tracing_subscriber::filter::LevelFilter::ERROR,
        _ => tracing_subscriber::filter::LevelFilter::INFO,
    });

    let subscriber = Registry::default()
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