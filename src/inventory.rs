use std::fs::{self, File};
use std::io::{Error, Write};

use log::{debug, info, log_enabled, Level};

pub struct InventoryPath {
    pub path: String,
}

pub trait ReadToString: Path {
    fn read_to_string(&self) -> Result<String, Error>;
}
pub trait Path {
    fn get_path(&self) -> String;
}

impl ReadToString for InventoryPath {
    fn read_to_string(&self) -> Result<String, Error> {
        if log_enabled!(Level::Info) {
            info!("Reading inventory {}", &self.path);
        }
        Ok(fs::read_to_string(&self.path)?)
    }
}

impl Path for InventoryPath {
    fn get_path(&self) -> String {
        self.path.clone()
    }
}

pub fn list(read_to_string_impl: &dyn ReadToString) -> Result<Vec<String>, Error> {
    let content = read_to_string_impl.read_to_string()?;

    let mut inv_list: Vec<String> = Vec::new();

    for line in content.lines() {
        inv_list.push(String::from(line));
    }

    if log_enabled!(Level::Info) {
        info!(
            "Finished listing inventory file {} with count {}",
            read_to_string_impl.get_path(),
            inv_list.len()
        );
    }

    Ok(inv_list)
}

pub trait Append: Path {
    fn append(&self, new_content: &String) -> Result<(), Error>;
}

impl Append for InventoryPath {
    fn append(&self, new_content: &String) -> Result<(), Error> {
        if log_enabled!(Level::Debug) {
            debug!("Appending inventory {}", &self.path);
        }
        let mut option = fs::OpenOptions::new();
        let mut file: File = option.create(true).append(true).open(&self.path)?;

        Ok(file.write_all(new_content.as_bytes())?)
    }
}

pub fn append_one(append_impl: &dyn Append, uploaded: &str) -> Result<bool, Error> {
    if uploaded.is_empty() {
        return Ok(false);
    }

    let new_content = format!("{}\n", uploaded);
    append_impl.append(&new_content)?;

    if log_enabled!(Level::Debug) {
        debug!("Appended inventory file {}", append_impl.get_path())
    }

    Ok(true)
}

#[cfg(test)]
mod tests {

    use super::*;

    struct TestInventoryPath {
        new_content: String,
        expected_content: String,
    }

    impl ReadToString for TestInventoryPath {
        fn read_to_string(&self) -> Result<String, Error> {
            Ok(String::from(&self.new_content))
        }
    }
    impl Path for TestInventoryPath {
        fn get_path(&self) -> String {
            todo!()
        }
    }

    #[test]
    fn list_inventory() {
        let expected: Vec<String> = vec!["line1".into(), "line2".into()];
        let test_inventory_file = TestInventoryPath {
            new_content: expected.join("\n"),
            expected_content: "".into(),
        };
        assert_eq!(list(&test_inventory_file).unwrap(), expected);
    }

    impl Append for TestInventoryPath {
        fn append(&self, new_content: &String) -> Result<(), Error> {
            assert_eq!(new_content, &self.expected_content);
            Ok(())
        }
    }

    #[test]
    fn append_one_inserts_new_line() {
        let uploaded = "img1";

        let expected = format!("{}\n", uploaded);

        let test_inventory = TestInventoryPath {
            new_content: uploaded.into(),
            expected_content: expected.into(),
        };

        assert_eq!(append_one(&test_inventory, &uploaded).unwrap(), true);
    }

    #[test]
    fn append_one_skips_when_uploaded_is_empty() {
        let uploaded = "";
        let test_inventory = TestInventoryPath {
            new_content: "".into(),
            expected_content: "".into(),
        };

        assert_eq!(append_one(&test_inventory, &uploaded).unwrap(), false);
    }
}
