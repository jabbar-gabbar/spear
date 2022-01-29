use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct ReadToStringResult {
    file_content: String,
}

pub trait ReadToString {
    fn read_to_string(&self) -> Result<ReadToStringResult, Error>;
}

pub struct InventoryFile {
    pub inventory_path: String,
}

impl ReadToString for InventoryFile {
    fn read_to_string(&self) -> Result<ReadToStringResult, Error> {
        Ok(ReadToStringResult {
            file_content: fs::read_to_string(&self.inventory_path)?,
        })
    }
}

pub fn list(read_to_string_impl: &dyn ReadToString) -> Result<Vec<String>, Error> {
    let content: ReadToStringResult = read_to_string_impl.read_to_string()?;

    let mut lines: Vec<String> = Vec::new();

    for line in content.file_content.lines() {
        lines.push(String::from(line));
    }

    Ok(lines)
}

pub fn append(inventory_path: String) -> Result<(), Error> {
    if !Path::new(&inventory_path).exists() {
        let parent_dir = match Path::new(&inventory_path).parent() {
            Some(p) => p,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Inventory path terminates at root",
                ));
            }
        };

        match fs::create_dir_all(parent_dir) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    let mut option = fs::OpenOptions::new();
    let mut file: File = option.append(true).create(true).open(inventory_path)?;
    Ok(file.write_all("text\n".as_bytes())?)
}

struct TestInventoryFile {
    content: String,
}
impl ReadToString for TestInventoryFile {
    fn read_to_string(&self) -> Result<ReadToStringResult, Error> {
        Ok(ReadToStringResult {
            file_content: String::from(&self.content),
        })
    }
}

#[cfg(inventory_tests)]
#[test]
fn list_returns_list() {
    let expected: Vec<String> = vec!["test1".into(), "test2".into()];
    let test_inventory_file = TestInventoryFile {
        content: expected.join("\r\n"),
    };
    assert_eq!(list(&test_inventory_file).unwrap(), expected);
}
