use std::{collections::HashSet, path::Path};

pub fn filter(extension_filter: &str, source: Vec<String>) -> Vec<String> {
    let mut filtered = vec![];

    let sanitized: String = sanitize(extension_filter);

    let exts: Vec<&str> = sanitized.split(',').collect();

    let mut excluded: HashSet<&str> = HashSet::new();

    for ext in exts {
        excluded.insert(ext);
    }

    for file_path in source {
        match Path::new(&file_path).extension() {
            Some(ext) => {
                let file_ext = ext.to_str().unwrap_or("").to_lowercase();
                if !excluded.contains(file_ext.as_str()) {
                    filtered.push(file_path);
                }
            }
            None => {}
        }
    }

    filtered
}

fn sanitize(filter: &str) -> String {
    filter
        .to_lowercase()
        .replace(".", "")
        .split_whitespace()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_it() {
        let extensions = "tmp,txt";
        let source = vec![
            "file.img".to_string(),
            "file.tmp".to_string(),
            "file.txt".to_string(),
        ];
        let expected = vec!["file.img".to_string()];
        let result = filter(extensions, source);
        assert_eq!(result, expected);
    }

    #[test]
    fn whitespace_extension() {
        let extensions = "tmp, ,txt";
        let source = vec![
            "file.img".to_string(),
            "file.tmp".to_string(),
            "file.txt".to_string(),
        ];
        let expected = vec!["file.img".to_string()];
        let result = filter(extensions, source);
        assert_eq!(result, expected);
    }

    #[test]
    fn whitespace_in_extension() {
        let extensions = "tmp,  txt";
        let source = vec![
            "file.img".to_string(),
            "file.tmp".to_string(),
            "file.txt".to_string(),
        ];
        let expected = vec!["file.img".to_string()];
        let result = filter(extensions, source);
        assert_eq!(result, expected);
    }

    #[test]
    fn dot_in_extension() {
        let extensions = ".tmp,txt";
        let source = vec![
            "file.img".to_string(),
            "file.tmp".to_string(),
            "file.txt".to_string(),
        ];
        let expected = vec!["file.img".to_string()];
        let result = filter(extensions, source);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_insensitive_extension() {
        let extensions = "TMP, txt";
        let source = vec![
            "file.img".to_string(),
            "file.tmp".to_string(),
            "file.TXT".to_string(),
        ];
        let expected = vec!["file.img".to_string()];
        let result = filter(extensions, source);
        assert_eq!(result, expected);
    }
}
