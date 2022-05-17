use log::{info, log_enabled, Level};
use std::{io::Error, path::Path};
use walkdir::WalkDir;

use crate::inventory::Path as DirPath;

/// Returns list of absolute file paths in the source directory recursively.
///
/// This function will include file paths recursively (in current and sub-directories) and
/// exclude any directory paths in the result `Vec<String>`.
pub fn list(dir_reader: &dyn ReadDir) -> Result<Vec<String>, Error> {
    let mut source_files: Vec<String> = vec![];

    for path in dir_reader.ls()? {
        if dir_reader.is_file(&path) {
            source_files.push(path);
        }
    }

    if log_enabled!(Level::Info) {
        info!(
            "Read source dir {} with count {}",
            dir_reader.get_path(),
            source_files.len()
        );
    }

    Ok(source_files)
}

pub struct SourceDir {
    pub dir_path: String,
}

pub trait ReadDir: DirPath {
    /// Lists objects in a directory recursively and returns `Vec<String>` of absolute
    /// paths of objects wrapped in `Result<T,E>`
    fn ls(&self) -> Result<Vec<String>, Error>;
    /// Returns `true` if the absolute path points to a file on disk
    fn is_file(&self, path: &str) -> bool;
}

impl ReadDir for SourceDir {
    fn ls(&self) -> Result<Vec<String>, Error> {
        if log_enabled!(Level::Info) {
            info!("Recursively reading source dir {}", &self.dir_path);
        }

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

impl DirPath for SourceDir {
    fn get_path(&self) -> String {
        self.dir_path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn list_it() {
        let iter: Vec<String> = vec!["/dir1/file1.ext".into(), "/dir1/file2.ext".into()];
        let expected = iter.clone();
        let dir = TestSourceDir { iter: iter };

        let result = list(&dir).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn list_filters_dir_paths() {
        let iter: Vec<String> = vec![
            "/dir1/file1.ext".into(),
            "/dir1/file2.ext".into(),
            "/dir1/sub_dir".into(),
        ];

        let mut filtered: Vec<String> = vec![];
        for i in &iter {
            if is_file(i) {
                filtered.push(i.to_string());
            }
        }

        let dir = TestSourceDir { iter: iter };

        let result = list(&dir).unwrap();

        assert_eq!(result, filtered);
    }

    struct TestSourceDir {
        iter: Vec<String>,
    }

    impl ReadDir for TestSourceDir {
        fn ls(&self) -> Result<Vec<String>, Error> {
            Ok(self.iter.to_vec())
        }

        fn is_file(&self, path: &str) -> bool {
            is_file(path)
        }
    }
    impl DirPath for TestSourceDir {
        fn get_path(&self) -> String {
            todo!()
        }
    }

    fn is_file(path: &str) -> bool {
        Path::new(path).extension().is_some()
    }
}
