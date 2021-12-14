use config::{Config, ConfigError, File};
use log::debug;

#[derive(Debug, Deserialize)]
pub struct SpearDefault {
    pub file_location: String,
    pub greetings: String,
    pub fish_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub spear_default: SpearDefault,
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
