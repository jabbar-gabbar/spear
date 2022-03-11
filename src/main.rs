use crate::inventory::InventoryPath;
use log::error;
use spear::inventory;
use spear::settings::Settings;
use std::process;

fn main() {
    env_logger::init();

    let _settings = Settings::default().unwrap_or_else(|err| {
        error!("{}", err);
        process::exit(1);
    });

    let inventory_file = InventoryPath {
        path: String::from("inventory"),
    };

    let mut new_inventory = String::from("new line");

    match inventory::append(&inventory_file, &mut new_inventory) {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    }

    match inventory::list(&inventory_file) {
        Ok(lines) => {
            for l in lines {
                println!("{:?}", l);
            }
        }
        Err(e) => error!("{}", e),
    }
}
