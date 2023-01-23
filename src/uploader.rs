use log::{debug, error, info, log_enabled, Level};

use crate::{
    aws_s3::AwsS3,
    inventory::{self, Append},
    prepare_upload::UploadItem,
};

pub async fn upload_one(aws_s3: &dyn AwsS3, upload: &UploadItem, s3_bucket: &str) -> bool {
    debug!(
        "Uploading file {} to s3 bucket {} with key {}",
        upload.file_path(),
        s3_bucket,
        upload.object_key_name()
    );
    aws_s3.put_object(s3_bucket, upload.object_key_name(), upload.file_path()).await
}

pub async fn upload(
    aws_s3: &dyn AwsS3,
    uploads: &Vec<UploadItem>,
    s3_bucket: &str,
    append_impl: &dyn Append,
) -> Vec<String> {
    let mut uploaded = vec![];
    if log_enabled!(Level::Info) {
        info!("Uploading {} objects to {}", uploads.len(), s3_bucket);
    }

    for upload in uploads {
        debug!(
            "Uploading file {} to s3 bucket with key {}/{}",
            upload.file_path(),
            s3_bucket,
            upload.object_key_name()
        );
        if aws_s3
            .put_object(s3_bucket, upload.object_key_name(), upload.file_path())
            .await
        {
            uploaded.push(upload.object_key_name().to_string());

            match inventory::append_one(append_impl, &upload.file_path()) {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        "Could not append inventory file {} with {}",
                        append_impl.get_path(),
                        &upload.file_path()
                    );
                    error!("{}", e);
                }
            }
        }
    }
    uploaded
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::{collections::HashMap, io::Error};

    use crate::inventory::Path;

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

        let test_inventory = TestInventoryPath {};

        // Action
        let uploaded = upload(&test_aws_s3, &upload_items, "s3_bucket", &test_inventory).await;

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
