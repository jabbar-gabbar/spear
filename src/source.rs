use std::{io::Error, path::Path};
use walkdir::WalkDir;

pub fn list(dir: &dyn Dir) -> Result<Vec<String>, Error> {
    let mut source_file_list: Vec<String> = vec![];

    for path in dir.iter()? {
        if dir.is_file(&path) {
            source_file_list.push(path);
        }
    }
    Ok(source_file_list)
}

pub struct SourceDir {
    pub dir_path: String,
}
pub trait Dir {
    fn iter(&self) -> Result<Vec<String>, Error>;
    fn is_file(&self, path: &str) -> bool;
}

impl Dir for SourceDir {
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
