use crate::inventory::InventoryPath;
use log::error;
use spear::prepare_upload::prepare;
use spear::settings::Settings;
use spear::{inventory, source};
use std::process;

fn main() {
    env_logger::init();

    let source_dir = source::SourceDir {
        dir_path: ".".into(),
    };
    if let Ok(source_paths) = source::list(&source_dir) {
        let uploads = prepare(
            source_paths.iter().map(|f| f.as_str()).collect(),
            vec![],
            &source_dir.dir_path,
        );
        for u in uploads {
            println!("keys: {}", u.object_key_name());
        }

        for source_path in source_paths {
            println!("source file: {}", source_path);
        }
    }

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
