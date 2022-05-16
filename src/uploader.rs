use log::{debug, info, log_enabled, Level};

use crate::{aws_s3::AwsS3, prepare_upload::UploadItem};

pub async fn upload(aws_s3: &dyn AwsS3, items: &Vec<UploadItem>, s3_bucket: &str) -> Vec<String> {
    let mut uploaded = vec![];
    if log_enabled!(Level::Info) {
        info!("Uploading {} objects to {}", items.len(), s3_bucket);
    }

    for item in items {
        debug!(
            "Uploading file {} to s3 bucket with key {}/{}",
            item.file_path(),
            s3_bucket,
            item.object_key_name()
        );
        if aws_s3
            .put_object(s3_bucket, item.object_key_name(), item.file_path())
            .await
        {
            uploaded.push(item.object_key_name().to_string());
        }
    }
    uploaded
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    async fn upload_it() {
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

        let test_aws_s3 = TestS3Client { map };

        // Action
        let uploaded = upload(&test_aws_s3, &upload_items, "s3_bucket").await;

        // Test
        assert_eq!(uploaded, expected);
    }

    struct TestS3Client {
        map: HashMap<String, bool>,
    }

    #[async_trait]
    impl AwsS3 for TestS3Client {
        async fn put_object(&self, _bucket: &str, key: &str, _file: &str) -> bool {
            if let Some(&b) = self.map.get(key) {
                return b;
            }
            false
        }
    }
}
