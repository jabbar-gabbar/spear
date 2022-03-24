use std::{io::Error, path::Path};
use walkdir::WalkDir;

pub fn list(dir_reader: &dyn ReadDir) -> Result<Vec<String>, Error> {
    let mut source_files: Vec<String> = vec![];

    for path in dir_reader.iter()? {
        if dir_reader.is_file(&path) {
            source_files.push(path);
        }
    }
    Ok(source_files)
}

pub struct SourceDir {
    pub dir_path: String,
}
pub trait ReadDir {
    fn iter(&self) -> Result<Vec<String>, Error>;
    fn is_file(&self, path: &str) -> bool;
}

impl ReadDir for SourceDir {
    fn iter(&self) -> Result<Vec<String>, Error> {
        let mut list: Vec<String> = Vec::new();
        for entry in WalkDir::new(&self.dir_path) {
            let entry = entry?;
            if let Some(path) = entry.path().to_str() {
                list.push(String::from(path));
            }
        }
        Ok(list)
    }

    fn is_file(&self, path: &str) -> bool {
        Path::new(path).is_file()
    }
}
