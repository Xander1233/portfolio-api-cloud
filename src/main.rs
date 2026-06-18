mod app;
mod cache;
mod config;
mod error;
mod logging;
mod models;
mod repo;
mod routes;
mod state;

use aws_config::{BehaviorVersion, Region};

use crate::config::{AwsSecrets, CONFIG};
use crate::logging::init_tracing;

const AWS_REGION: &str = "eu-central-1";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _log_guard = init_tracing();

    let config = (*CONFIG).clone();

    tracing::info!(region = AWS_REGION, "loading AWS shared config");
    let shared_config = aws_config::defaults(BehaviorVersion::v2026_01_12())
        .region(Region::new(AWS_REGION))
        .load()
        .await;

    let secrets = match AwsSecrets::load_from_aws(&shared_config).await {
        Ok(secrets) => {
            tracing::info!(
                table = %secrets.ddb_table,
                "AWS secrets loaded"
            );
            secrets
        }
        Err(err) => {
            tracing::error!(error = %err, "failed to load AWS secrets");
            panic!("failed to load AWS secrets: {err}");
        }
    };

    app::run(config, secrets, shared_config).await
}
