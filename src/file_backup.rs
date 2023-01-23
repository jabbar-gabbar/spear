use crate::{
    aws_s3::AwsS3,
    inventory::{self, Append},
    prepare_upload::UploadItem,
    uploader,
};
use log::error;

pub async fn backup(
    aws_s3: &dyn AwsS3,
    uploads: &Vec<UploadItem>,
    s3_bucket: &str,
    append_impl: &dyn Append,
) -> usize {
    let mut count = 0;
    for upload in uploads {
        if uploader::upload_one(aws_s3, upload, s3_bucket).await {
            match inventory::append_one(append_impl, &upload.object_key_name()) {
                Ok(_) => {
                    count += 1;
                }
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
    count
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        io::{Error, ErrorKind},
    };

    use async_trait::async_trait;

    use crate::{
        aws_s3::AwsS3,
        file_backup::backup,
        inventory::{Append, Path},
        prepare_upload::UploadItem,
    };

    #[tokio::test]
    async fn backup_works_entirely() {
        // Setup
        let mut upload_items: Vec<UploadItem> = vec![];

        let key_name = "key_name";
        let upload_item = UploadItem::new("file_path".into(), key_name.to_string());
        upload_items.push(upload_item);

        let mut map: HashMap<String, bool> = HashMap::new();
        map.insert(key_name.to_string(), true);

        let mut map2: HashMap<String, bool> = HashMap::new();
        map2.insert(format!("{}\n", key_name.to_string()), true);

        let fake_aws_s3 = TestS3Client { map };
        let fake_append = TestInventoryPath { map: map2 };

        // Action
        let uploaded = backup(&fake_aws_s3, &upload_items, "s3_bucket", &fake_append).await;

        // Test
        assert_eq!(uploaded, 1);
    }

    #[tokio::test]
    async fn backup_partially_works_when_s3_fails() {
        // Setup
        let mut upload_items: Vec<UploadItem> = vec![];

        let mut s3_map: HashMap<String, bool> = HashMap::new();
        let mut append_map: HashMap<String, bool> = HashMap::new();

        for n in (0..2).step_by(1) {
            s3_map.insert(n.to_string(), n % 2 == 0); // s3 fails at on odd n
            append_map.insert(format!("{}\n", n.to_string()), true);

            let upload_item = UploadItem::new("file_path".into(), n.to_string());
            upload_items.push(upload_item);
        }

        let fake_aws_s3 = TestS3Client { map: s3_map };
        let fake_append = TestInventoryPath { map: append_map };

        // Action
        let uploaded = backup(&fake_aws_s3, &upload_items, "s3_bucket", &fake_append).await;

        // Test
        assert_eq!(uploaded, 1);
    }

    #[tokio::test]
    async fn backup_partially_works_when_inventory_append_fails() {
        // Setup
        let mut upload_items: Vec<UploadItem> = vec![];

        let mut s3_map: HashMap<String, bool> = HashMap::new();
        let mut append_map: HashMap<String, bool> = HashMap::new();

        for n in (0..2).step_by(1) {
            s3_map.insert(n.to_string(), true);
            append_map.insert(format!("{}\n", n.to_string()), n % 2 == 0); // append fails at odd n

            let upload_item = UploadItem::new("file_path".into(), n.to_string());
            upload_items.push(upload_item);
        }

        let fake_aws_s3 = TestS3Client { map: s3_map };
        let fake_append = TestInventoryPath { map: append_map };

        // Action
        let uploaded = backup(&fake_aws_s3, &upload_items, "s3_bucket", &fake_append).await;

        // Test
        assert_eq!(uploaded, 1);
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

    struct TestInventoryPath {
        map: HashMap<String, bool>,
    }

    impl Path for TestInventoryPath {
        fn get_path(&self) -> String {
            todo!()
        }
    }

    impl Append for TestInventoryPath {
        fn append(&self, new_content: &String) -> Result<(), Error> {
            if let Some(&val) = self.map.get(new_content.as_str()) {
                if val {
                    return Ok(());
                }
            }
            Err(Error::new(ErrorKind::Other, "uh-oh"))
        }
    }
}
