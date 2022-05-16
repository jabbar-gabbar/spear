use config::{ConfigError, File};
use log::debug;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Backup {
    source_directory_path: String,
    s3_bucket: String,
    inventory_file_path: String,
}

impl Backup {
    /// Get a reference to the backup's source directory path.
    #[must_use]
    pub fn source_directory_path(&self) -> &str {
        self.source_directory_path.as_ref()
    }

    /// Get a reference to the backup's s3 bucket.
    #[must_use]
    pub fn s3_bucket(&self) -> &str {
        self.s3_bucket.as_ref()
    }

    /// Get a reference to the backup's inventory file path.
    #[must_use]
    pub fn inventory_file_path(&self) -> &str {
        self.inventory_file_path.as_ref()
    }
}
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub backup: Vec<Backup>,
}

impl Settings {
    /// Returns `Settings` from Settings.toml configuration file
    pub fn default() -> Result<Self, ConfigError> {
        Settings::from("Settings")
    }
    /// Returns 'Settings' from `file_name`
    pub fn from(file_name: &str) -> Result<Self, ConfigError> {
        debug!("Reading settings from {}", file_name);
        let mut cfg = config::Config::default();
        cfg.merge(File::with_name(&file_name))?;
        cfg.try_into::<Settings>()
    }
}
