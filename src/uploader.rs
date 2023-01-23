use log::debug;

use crate::{aws_s3::AwsS3, prepare_upload::UploadItem};

pub async fn upload_one(aws_s3: &dyn AwsS3, upload: &UploadItem, s3_bucket: &str) -> bool {
    debug!(
        "Uploading file {} to s3 bucket {} with key {}",
        upload.file_path(),
        s3_bucket,
        upload.object_key_name()
    );
    aws_s3
        .put_object(s3_bucket, upload.object_key_name(), upload.file_path())
        .await
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::{collections::HashMap, io::Error};

    use crate::inventory::{Append, Path};

    use super::*;

    #[tokio::test]
    async fn upload_it() {
        // Set up
        let upload_item = UploadItem::new("file_path".into(), "key_name".into());
        let mut map: HashMap<String, bool> = HashMap::new();
        map.insert(upload_item.object_key_name().to_string(), true);

        let test_aws_s3 = TestS3Client { map };

        // Action
        let uploaded = upload_one(&test_aws_s3, &upload_item, "s3_bucket").await;

        // Test
        assert_eq!(uploaded, true);
    }

    #[tokio::test]
    async fn upload_fails() {
        // Set up
        let upload_item = UploadItem::new("file_path".into(), "key_name".into());
        let mut map: HashMap<String, bool> = HashMap::new();
        map.insert(upload_item.object_key_name().to_string(), false);

        let test_aws_s3 = TestS3Client { map };

        // Action
        let uploaded = upload_one(&test_aws_s3, &upload_item, "s3_bucket").await;

        // Test
        assert_eq!(uploaded, false);
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

    struct TestInventoryPath {}

    impl Path for TestInventoryPath {
        fn get_path(&self) -> String {
            todo!()
        }
    }

    impl Append for TestInventoryPath {
        fn append(&self, _new_content: &String) -> Result<(), Error> {
            Ok(())
        }
    }
}
