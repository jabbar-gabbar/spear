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

pub struct FileResult {
    file: File,
}

pub trait Append {
    fn open(&self) -> Result<FileResult, Error>;
    fn write_all(&self, opened_file: FileResult, append_content: &String) -> Result<(), Error>;
}

impl Append for InventoryPath {
    fn open(&self) -> Result<FileResult, Error> {
        let mut option = fs::OpenOptions::new();

        let file: File = option.append(true).create(true).open(&self.path)?;

        Ok(FileResult { file: file })
    }

    fn write_all(&self, mut opened_file: FileResult, append_content: &String) -> Result<(), Error> {
        Ok(opened_file.file.write_all(append_content.as_bytes())?)
    }
}

pub fn append(append_impl: &dyn Append, new_content: &mut String) -> Result<(), Error> {
    let file = append_impl.open()?;

    new_content.push_str("\n");

    Ok(append_impl.write_all(file, new_content)?)
}

#[cfg(inventory_tests)]
mod tests {

    struct TestInventoryFile {
        content: String,
    }
    impl ReadToString for TestInventoryFile {
        fn read_to_string(&self) -> Result<String, Error> {
            Ok(String::from(&self.content))
        }
    }

    #[test]
    fn list_inventory() {
        let expected: Vec<String> = vec!["test1".into(), "test2".into()];
        let test_inventory_file = TestInventoryFile {
            content: expected.join("\r\n"),
        };
        assert_eq!(list(&test_inventory_file).unwrap(), expected);
    }
}
