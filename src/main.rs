use log::{error};
use spear::{file_backup, settings::Settings};
use std::process;

fn main() {
    env_logger::init();

    let settings = Settings::default().unwrap_or_else(|err| {
        error!("{}", err);
        process::exit(1);
    });

    file_backup::run(settings);
}
