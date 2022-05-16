use log::{error, info, log_enabled, Level};

use crate::{
    aws_s3::AwsS3,
    inventory::{self, InventoryPath},
    prepare_upload,
    settings::Settings,
    source::{self, SourceDir},
    uploader,
};

pub async fn run(settings: Settings, aws_s3: &dyn AwsS3) {
    for backup in settings.backup {
        if log_enabled!(Level::Info) {
            log_backing_up(
                backup.source_directory_path(),
                backup.s3_bucket(),
                backup.inventory_file_path(),
            );
        }

        let inv_path = InventoryPath {
            path: backup.inventory_file_path().to_string(),
        };

        let inventory = match inventory::list(&inv_path) {
            Ok(inv) => inv,
            Err(e) => {
                log_error(
                    &format!(
                        "Could not read inventory file at {}",
                        backup.inventory_file_path()
                    ),
                    &e.to_string(),
                );
                continue;
            }
        };

        let source_dir = SourceDir {
            dir_path: backup.source_directory_path().to_string(),
        };

        let source = match source::list(&source_dir) {
            Ok(src) => src,
            Err(e) => {
                log_error(
                    &format!(
                        "Could not read source dir at {}",
                        backup.source_directory_path()
                    ),
                    &e.to_string(),
                );
                continue;
            }
        };

        let prepared = prepare_upload::prepare(&source, &inventory, backup.source_directory_path());

        let uploaded = uploader::upload(aws_s3, &prepared, backup.s3_bucket()).await;

        match inventory::append(&inv_path, &uploaded) {
            Ok(_) => {}
            Err(e) => log_error("Could not append inventory {}", &e.to_string()),
        }

        if log_enabled!(Level::Info) {
            log_metric(
                backup.source_directory_path(),
                backup.s3_bucket(),
                source.len(),
                inventory.len(),
                prepared.len(),
                uploaded.len(),
            )
        }
    }

    if log_enabled!(Level::Info) {
        info!("Backup complete!");
    }
}

fn log_backing_up(dir: &str, bucket: &str, inventory: &str) {
    info!("---- Backing up dir: {} --> bucket: {} ----", dir, bucket);
    info!("---- Using inventory file : {}", inventory);
}

fn log_metric(
    dir: &str,
    bucket: &str,
    source: usize,
    inv: usize,
    prepared: usize,
    uploaded: usize,
) {
    info!("---- {} --> bucket: {} ----", dir, bucket);
    info!(
        "---- Metric source: {}, inventory: {}, prepared: {}, uploaded: {} ----",
        source, inv, prepared, uploaded
    );
}

/// Log custom error
fn log_error(msg: &str, err: &str) {
    error!("{}", msg);
    error!("{}", err);
}
