use std::io::Error;
use walkdir::WalkDir;

pub fn list(source_directory: &str) -> Result<Vec<String>, Error> {
    let mut source_file_list: Vec<String> = Vec::new();

    for entry in WalkDir::new(source_directory) {
        let entry = entry?;

        if !entry.path().is_dir() {
            if let Some(path) = entry.path().to_str() {
                source_file_list.push(String::from(path));
            }
        }
    }
    Ok(source_file_list)
}
