use log::{error, info, log_enabled, Level};

use crate::{
    aws_s3::AwsS3,
    file_backup, filter,
    inventory::{self, InventoryPath},
    prepare_upload,
    settings::Settings,
    source::{self, SourceDir},
};

pub async fn run(settings: Settings, aws_s3: &dyn AwsS3) {
    for config in settings.backup {
        if log_enabled!(Level::Info) {
            log_backing_up(
                config.source_directory_path(),
                config.s3_bucket(),
                config.inventory_file_path(),
            );
        }

        let inv_path = InventoryPath {
            path: config.inventory_file_path().to_string(),
        };

        let inventory = match inventory::list(&inv_path) {
            Ok(inv) => inv,
            Err(e) => {
                log_error(
                    &format!(
                        "Could not read inventory file at {}",
                        config.inventory_file_path()
                    ),
                    &e.to_string(),
                );
                continue;
            }
        };

        let source_dir = SourceDir {
            dir_path: config.source_directory_path().to_string(),
        };

        let source = match source::list(&source_dir) {
            Ok(src) => src,
            Err(e) => {
                log_error(
                    &format!(
                        "Could not read source dir at {}",
                        config.source_directory_path()
                    ),
                    &e.to_string(),
                );
                continue;
            }
        };

        let filtered = filter::filter(config.excluded_extensions(), source);

        let prepared =
            prepare_upload::prepare(&filtered, &inventory, config.source_directory_path());

        let count = file_backup::backup(aws_s3, &prepared, config.s3_bucket(), &inv_path).await;

        if log_enabled!(Level::Info) {
            log_metric(
                config.source_directory_path(),
                config.s3_bucket(),
                filtered.len(),
                inventory.len(),
                prepared.len(),
                count,
            )
        }
    }

    if log_enabled!(Level::Info) {
        info!("Backup complete!");
    }
}

fn log_backing_up(dir: &str, bucket: &str, inventory: &str) {
    info!("\n----");
    info!("\n----");
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
    info!("---- backed up: {} ==> bucket: {} ----", dir, bucket);
    info!(
        "---- source: {}, inventory: {}, prepared: {}, uploaded: {} ----",
        source, inv, prepared, uploaded
    );
}

/// Log custom error
fn log_error(msg: &str, err: &str) {
    error!("{}", msg);
    error!("{}", err);
}
