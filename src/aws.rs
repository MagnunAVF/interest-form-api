use aws_config::BehaviorVersion;
use once_cell::sync::OnceCell;

static AWS_CONFIG: OnceCell<aws_config::SdkConfig> = OnceCell::new();

pub async fn init_aws_config() {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    AWS_CONFIG
        .set(config)
        .expect("AWS config already initialized");
}

pub fn get_aws_config() -> &'static aws_config::SdkConfig {
    AWS_CONFIG.get().expect("AWS config not initialized")
}
