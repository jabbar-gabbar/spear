use aws_sdk_s3::Client;
use log::{error};
use spear::{file_backup, settings::Settings, aws_s3::S3Client};
use std::process;

#[tokio::main]
async fn main() {
    env_logger::init();

    let settings = Settings::default().unwrap_or_else(|err| {
        error!("{}", err);
        process::exit(1);
    });

    let config = aws_config::from_env().load().await;
    let client = Client::new(&config);
    let s3_client = S3Client{
        s3: client
    };

    file_backup::run(settings, &s3_client).await;
}
