use log::error;
use spear::Settings;
use std::process;

fn main() {
    env_logger::init();

    let _settings = Settings::default().unwrap_or_else(|err| {
        error!("{}", err);
        process::exit(1);
    });
}
