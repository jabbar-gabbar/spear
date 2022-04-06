use regex::Regex;
use std::{borrow::Borrow, collections::HashSet};

/// Prepares upload list from the source paths using inventory list and source dir path
pub fn prepare(
    file_paths: Vec<&str>,
    inventory_list: Vec<&str>,
    source_dir: &str,
) -> Vec<UploadItem> {
    let mut uploads: Vec<UploadItem> = vec![];

    let mut inv_set: HashSet<&str> = HashSet::new();
    for inv in inventory_list {
        inv_set.insert(inv);
    }

    for file_path in file_paths {
        if file_path.len() < source_dir.len() {
            continue;
        }
        let key_name = sanitize(&file_path[source_dir.len()..]);

        if let None = inv_set.get(&key_name.borrow()) {
            uploads.push(UploadItem {
                file_path: file_path.to_string(),
                object_key_name: key_name,
            });
        }
    }
    uploads
}

fn sanitize(suffix: &str) -> String {
    let mut suffix = suffix.replace("\\", "/");
    if let Some(first) = suffix.chars().nth(0) {
        if first == '/' {
            suffix.remove(0);
        }
    }
    // Replace chars that s3 does not support
    let reg = Regex::new(r"[{}%^`\[\]~<>#|]").unwrap();
    let sanitized = reg.replace_all(&suffix, "");

    sanitized.into_owned()
}

pub struct UploadItem {
    file_path: String,
    object_key_name: String,
}

impl UploadItem {
    pub fn new(file_path: String, object_key_name: String) -> Self {
        Self {
            file_path,
            object_key_name,
        }
    }

    /// Get a reference to the upload item's file path.
    #[must_use]
    pub fn file_path(&self) -> &str {
        self.file_path.as_ref()
    }

    /// Get a reference to the upload item's object key name.
    #[must_use]
    pub fn object_key_name(&self) -> &str {
        self.object_key_name.as_ref()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn prepare_it() {
        let file_paths: Vec<&str> = vec![
            "/top_dir/dir1/file1.ext",
            "/top_dir/dir1/dir2/file2.ext",
            "/top_dir/dir2/file3.ext",
        ];
        let inventory: Vec<&str> = vec![];
        let source_dir = "/top_dir/";

        let uploads = prepare(file_paths, inventory, source_dir);

        assert_eq!(3, uploads.len());
    }

    #[test]
    fn prepare_returns_empty() {
        let file_paths: Vec<&str> = vec!["/top_dir/dir1/file1.ext", "/top_dir/dir2/file3.ext"];
        let inventory: Vec<&str> = vec!["dir1/file1.ext", "dir2/file3.ext"];
        let source_dir = "/top_dir/";

        let uploads = prepare(file_paths, inventory, source_dir);

        assert_eq!(0, uploads.len());
    }

    #[test]
    fn prepare_removes_slash_from_0th_idx_in_object_key() {
        let file_paths: Vec<&str> = vec!["/top_dir/dir/file.ext"];
        let expected = "dir/file.ext";
        let inventory: Vec<&str> = vec![];
        let source_dir = "/top_dir";

        let uploads: Vec<UploadItem> = prepare(file_paths, inventory, source_dir);
        let actual = uploads
            .get(0)
            .map(|u| u.object_key_name())
            .unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn prepare_skips_shorter_file_paths_than_source_dir() {
        let only_file = "file1.ext";
        let file_paths: Vec<&str> = vec![&only_file];
        let inventory: Vec<&str> = vec![];
        let source_dir = "/top_dir/dir1";

        let uploads: Vec<UploadItem> = prepare(file_paths, inventory, source_dir);
        assert!(only_file.len() < source_dir.len());
        assert_eq!(uploads.len(), 0);
    }

    #[test]
    fn prepare_sanitizes_file_paths() {
        let file_paths: Vec<&str> = vec!["/top_dir/dir1/file1{}%^`[]>~<#|.ext"];
        let inventory: Vec<&str> = vec![];
        let source_dir = "/top_dir/";

        let uploads = prepare(file_paths, inventory, source_dir);

        assert_eq!(1, uploads.len());
        assert_eq!(
            "dir1/file1.ext",
            uploads
                .get(0)
                .map(|f| f.object_key_name())
                .unwrap_or_default()
        );
    }

    #[test]
    fn prepare_filters_inventory_paths() {
        let file_paths: Vec<&str> = vec!["/top_dir/dir1/file1.ext", "/top_dir/dir1/file2.ext"];
        let inventory: Vec<&str> = vec!["dir1/file1.ext"];
        let source_dir = "/top_dir/";
        let expected = "dir1/file2.ext";

        let uploads = prepare(file_paths, inventory, source_dir);
        assert_eq!(1, uploads.len());
        assert_eq!(
            expected,
            uploads
                .get(0)
                .map(|u| u.object_key_name())
                .unwrap_or_default()
        )
    }

    #[test]
    fn prepare_for_win_system() {
        let file_paths: Vec<&str> = vec![
            "c:\\\\top_dir\\dir1\\file1.ext",
            "c:\\\\top_dir\\dir1\\dir2\\file2.ext",
            "c:\\\\top_dir\\dir2\\file3.ext",
        ];
        let inventory: Vec<&str> = vec!["dir2/file3.ext"];
        let source_dir = "c:\\\\top_dir";

        let filtered_set: HashSet<&str> = inventory.iter().cloned().collect();

        let uploads = prepare(file_paths, inventory, source_dir);

        assert_eq!(2, uploads.len());
        for u in uploads {
            assert_eq!(None, filtered_set.get(&u.object_key_name()));
        }
    }
}
