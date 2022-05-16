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
        if log_enabled!(Level::Info) {
            info!("Appending inventory {}", &self.path);
        }
        let mut option = fs::OpenOptions::new();
        let mut file: File = option.create(true).append(true).open(&self.path)?;

        Ok(file.write_all(new_content.as_bytes())?)
    }
}

pub fn append(append_impl: &dyn Append, uploaded: &Vec<String>) -> Result<(), Error> {
    if uploaded.is_empty() {
        return Ok(());
    }
    let mut new_content = uploaded.join("\n");
    new_content.push_str("\n");
    append_impl.append(&new_content)?;

    if log_enabled!(Level::Info) {
        info!(
            "Appended inventory file {} with count {}",
            append_impl.get_path(),
            uploaded.len()
        );
    }

    Ok(())
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
    fn append_inserts_new_line() {
        let new_content = "";
        let uploaded = vec![new_content.to_string()];

        let mut expected = uploaded.join("\n");
        expected.push_str("\n");

        let test_inventory = TestInventoryPath {
            new_content: new_content.into(),
            expected_content: expected.into(),
        };

        append(&test_inventory, &uploaded).unwrap();
    }

    #[test]
    fn append_skips_when_uploaded_is_empty() {
        let uploaded = vec![];
        let test_inventory = TestInventoryPath {
            new_content: "".into(),
            expected_content: "".into(),
        };
        let result = append(&test_inventory, &uploaded).unwrap();
        assert_eq!(result, ());
    }
}
