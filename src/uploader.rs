use log::info;

use crate::{prepare_upload::UploadItem, s3_client::S3PutObject};

pub fn upload(
    s3_client: &dyn S3PutObject,
    items: &Vec<UploadItem>,
    s3_bucket: &str,
) -> Vec<String> {
    let mut uploaded = vec![];
    for item in items {
        info!(
            "Uploading file {} to s3 bucket with key {}/{}",
            item.file_path(),
            s3_bucket,
            item.object_key_name()
        );
        if s3_client.put_object(s3_bucket, item.object_key_name(), item.file_path()) {
            uploaded.push(item.object_key_name().to_string());
        }
    }
    uploaded
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn upload_it() {
        // Set up
        let mut upload_items: Vec<UploadItem> = vec![];
        let mut map: HashMap<String, bool> = HashMap::new();
        let mut expected: Vec<String> = vec![];

        for n in (1..4).step_by(1) {
            map.insert(n.to_string(), n % 2 == 0);

            if n % 2 == 0 {
                expected.push(n.to_string());
            }

            upload_items.push(UploadItem::new("file_path".into(), n.to_string()));
        }

        let test_s3_client = TestS3Client { map };

        // Action
        let uploaded = upload(&test_s3_client, &upload_items, "s3_bucket");

        // Test
        assert_eq!(uploaded, expected);
    }

    struct TestS3Client {
        map: HashMap<String, bool>,
    }

    impl S3PutObject for TestS3Client {
        fn put_object(&self, _bucket: &str, key: &str, _file: &str) -> bool {
            if let Some(&b) = self.map.get(key) {
                return b;
            }
            false
        }
    }
}
