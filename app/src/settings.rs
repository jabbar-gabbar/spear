use serde_derive::Deserialize;
use config::{Config, ConfigError, File};
use log::debug;

#[derive(Debug, Deserialize)]
pub struct Directory {
    pub source_directory: String,
    pub s3_bucket: String,
    pub inventory_file: String,
}

#[derive(Debug, Deserialize)]
pub struct Common {
    pub my_secrets: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub directory: Vec<Directory>,
    pub common: Common
}

impl Settings {
    /// Returns `Settings` from Settings.toml configuration file
    pub fn default() -> Result<Self, ConfigError> {
        Settings::from(String::from("Settings"))
    }
    /// Returns 'Settings' from `file_name`
    pub fn from(file_name: String) -> Result<Self, ConfigError> {
        debug!("Creating new instance of Settings using {}", file_name);
        let mut s: Config = config::Config::default();
        s.merge(File::with_name(&file_name))?;
        s.try_into::<Settings>()
    }
}
