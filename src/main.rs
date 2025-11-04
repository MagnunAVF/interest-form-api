use lambda_http::{run, service_fn, tracing, Error};
mod http_handler;
use http_handler::function_handler;

const ENV_VARS: [&str; 3] = ["ENV", "RUST_LOG", "RUST_BACKTRACE"];

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

    run(service_fn(function_handler)).await
}
