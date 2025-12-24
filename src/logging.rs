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

    let now = chrono::Utc::now().timestamp();

    let app_name = format!("{}-{}", config.general.hostname, &now);

    let stdout_layer = fmt::layer()
        .pretty()
        .with_target(false)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    std::fs::create_dir_all("logs")?;

    let mut log_files: Vec<_> = std::fs::read_dir("logs")?
        .filter_map(Result::ok)
        .filter(|entry| {
            if let Some(ext) = entry.path().extension() {
                ext == "log"
            } else {
                false
            }
        }).collect();
    let log_files_count = log_files.len();

    if log_files_count > 10 {
        log_files.sort_by_key(|entry| entry.metadata().and_then(|m| m.created()).unwrap_or(std::time::SystemTime::UNIX_EPOCH));
        if let Some(oldest) = log_files.first() {
            std::fs::remove_file(oldest.path())?;
        }
    }

    let file = std::fs::File::create(format!("logs/{app_name}.log"))?;

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