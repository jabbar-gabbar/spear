pub struct S3Client {
    pub s3: aws_sdk_s3::Client,
}
pub trait S3PutObject {
    fn put_object(&self, bucket: &str, key: &str, file: &str) -> bool;
}
impl S3PutObject for S3Client {
    fn put_object(&self, _bucket: &str, _key: &str, _file: &str) -> bool {
        true
    }
}
