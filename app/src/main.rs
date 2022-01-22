use spear::settings::Settings;
use crate::inventory::InventoryFile;
use spear::inventory;
use std::process;
use log::error;

fn main() {
    env_logger::init();

    let _settings = Settings::default().unwrap_or_else(|err| {
        error!("{}", err);
        process::exit(1);
    });

    match inventory::append(String::from("../inventory")){
        Ok(_)=>{},
        Err(e)=> error!("{}", e)
    }

    let inventory_file = InventoryFile {
        inventory_path: String::from("../inventory"),
    };

    match inventory::list(&inventory_file) {
        Ok(lines) => {
            for l in lines {
                println!("{:?}", l);
            }
        }
        Err(e) => error!("{}", e),
    }
}
