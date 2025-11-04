use http_handler::function_handler;
use lambda_http::{run, service_fn, tracing, Error};

mod aws;
mod db;
mod http_handler;
mod models;

const ENV_VARS: [&str; 5] = [
    "ENV",
    "RUST_LOG",
    "RUST_BACKTRACE",
    "INTERESTS_TABLE_NAME",
    "CUSTOM_AWS_REGION",
];

fn list_env_infos() {
    for &key in &ENV_VARS {
        match std::env::var(key) {
            Ok(val) => tracing::info!("[ENV VARS] {} = {}", key, val),
            Err(_) => tracing::info!("[ENV VARS] {} not set", key),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    tracing::debug!("[FUNCTION] Starting api in debug mode in app!");

    list_env_infos();

    aws::init_aws_config().await;

    run(service_fn(function_handler)).await
}
