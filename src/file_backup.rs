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
