use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Write;

pub struct InventoryPath {
    pub path: String,
}

pub trait ReadToString {
    fn read_to_string(&self) -> Result<String, Error>;
}

impl ReadToString for InventoryPath {
    fn read_to_string(&self) -> Result<String, Error> {
        Ok(fs::read_to_string(&self.path)?)
    }
}

pub fn list(read_to_string_impl: &dyn ReadToString) -> Result<Vec<String>, Error> {
    let content = read_to_string_impl.read_to_string()?;

    let mut lines: Vec<String> = Vec::new();

    for line in content.lines() {
        lines.push(String::from(line));
    }

    Ok(lines)
}

pub trait Append {
    fn append(&self, new_content: &String) -> Result<(), Error>;
}

impl Append for InventoryPath {
    fn append(&self, new_content: &String) -> Result<(), Error> {
        let mut option = fs::OpenOptions::new();
        let mut file: File = option.create(true).append(true).open(&self.path)?;

        Ok(file.write_all(new_content.as_bytes())?)
    }
}

pub fn append(append_impl: &dyn Append, new_content: &mut String) -> Result<(), Error> {
    new_content.push_str("\n");
    Ok(append_impl.append(new_content)?)
}

#[cfg(test)]
mod tests {

    use super::*;

    struct TestInventoryPath {
        content: String,
    }
    impl ReadToString for TestInventoryPath {
        fn read_to_string(&self) -> Result<String, Error> {
            Ok(String::from(&self.content))
        }
    }

    #[test]
    fn list_inventory() {
        let expected: Vec<String> = vec!["line1".into(), "line2".into()];
        let test_inventory_file = TestInventoryPath {
            content: expected.join("\r\n"),
        };
        assert_eq!(list(&test_inventory_file).unwrap(), expected);
    }

    impl Append for TestInventoryPath {
        fn append(&self, _: &String) -> Result<(), Error> {
            Ok(())
        }
    }

    #[test]
    fn append_inserts_new_line() {
        let test_inventory_path = TestInventoryPath { content: "".into() };

        let mut new_content: String = "".into();
        let mut expected: String = new_content.clone();
        expected.push_str("\n");

        append(&test_inventory_path, &mut new_content).unwrap();

        assert_eq!(new_content, expected);
    }
}
