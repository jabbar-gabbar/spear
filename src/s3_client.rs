pub struct S3Client {
    pub s3: String,
}
pub trait S3PutObject {
    fn put_object(&self, bucket: &str, key: &str, file: &str) -> bool;
}
impl S3PutObject for S3Client {
    fn put_object(&self, _bucket: &str, _key: &str, _file: &str) -> bool {
        true
    }
}
