use std::path::Path;

use async_trait::async_trait;
use aws_sdk_s3::{types::ByteStream, Client};
use log::error;

pub struct S3Client {
    pub s3: Client,
}

#[async_trait]
pub trait AwsS3 {
    async fn put_object(&self, bucket: &str, key: &str, file: &str) -> bool;
}

#[async_trait]
impl AwsS3 for S3Client {
    async fn put_object(&self, bucket: &str, key: &str, file_path: &str) -> bool {
        let body = ByteStream::from_path(Path::new(file_path)).await;

        match &self
            .s3
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body.unwrap())
            .send()
            .await
        {
            Ok(_) => {
                return true;
            }
            Err(e) => {
                error!("Error uploading {},\n\r{}", key, e);
                return false;
            }
        }
    }
}
