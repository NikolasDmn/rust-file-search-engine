use std::collections::HashMap;
use std::path::Path;
use std::fs;

use crate::file_entry::FileEntry;

// Get the files in a directory
fn get_files(path: &Path) -> Vec<String> {
    let mut files = vec![];
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return vec![],
    };

    for entry in entries.flatten() {
        let entry_path = entry.path();
        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };

        if file_type.is_dir() {
            files.extend(get_files(&entry_path));
        } else if file_type.is_file() {
            if let Some(file_name) = entry_path.to_str() {
                files.push(file_name.to_owned());
            }
        }
    }

    files
}
pub fn get_file_set(path: &Path) -> HashMap<usize, FileEntry> {
    get_files(path)
        .into_iter()
        .enumerate()
        .map(|(id, file)| (id, FileEntry::new(id, Path::new(&file))))
        .collect()
}

// Get the content of a file

pub fn get_file_content(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn get_file_title(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default()
        .to_string()
}

#[cfg(test)]
mod get_file_sets {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    fn create_mock_dir_structure() -> (tempfile::TempDir, std::path::PathBuf) {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_path_buf(); // Clone the path here

        // Creating subdirectories and files
        fs::create_dir_all(dir_path.join("subdir1")).unwrap();
        File::create(dir_path.join("subdir1/file1.txt")).unwrap();
        File::create(dir_path.join("subdir1/file2.txt")).unwrap();
        fs::create_dir_all(dir_path.join("subdir2")).unwrap();
        File::create(dir_path.join("subdir2/file3.txt")).unwrap();
        File::create(dir_path.join("file4.txt")).unwrap();

        (temp_dir, dir_path)
    }



    #[test]
    fn test_get_files_with_valid_directory() {
        let (_temp_dir, dir_path) = create_mock_dir_structure();
        let files = get_files(&dir_path);
        println!("{:?}", files);
        assert_eq!(files.len(), 4);
    }

    #[test]
    fn test_get_files_with_non_existent_directory() {
        let dir_path = Path::new("/non/existent/directory");
        let files = get_files(&dir_path);
        assert!(files.is_empty());
    }

    #[test]
    fn test_get_files_with_empty_directory() {
        let dir = tempdir().unwrap();
        let files = get_files(&dir.path());
        assert!(files.is_empty());
    }
}


#[cfg(test)]
mod get_file_set_tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    fn create_mock_dir_structure() -> (tempfile::TempDir, std::path::PathBuf) {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_path_buf(); // Clone the path here

        // Creating subdirectories and files
        File::create(dir_path.join("file1.txt")).unwrap();
        File::create(dir_path.join("file2.txt")).unwrap();

        (temp_dir, dir_path)
    }

    #[test]
    fn test_get_file_set_with_valid_directory() {
        let (_temp_dir, dir_path) = create_mock_dir_structure();
        let file_set = get_file_set(&dir_path);
        assert_eq!(file_set.len(), 2);
    }

    #[test]
    fn test_get_file_set_with_empty_directory() {
        let dir = tempdir().unwrap();
        let file_set = get_file_set(dir.path());
        assert!(file_set.is_empty());
    }

    #[test]
    fn test_get_file_set_with_non_existent_directory() {
        let dir_path = Path::new("/non/existent/directory");
        let file_set = get_file_set(&dir_path);
        assert!(file_set.is_empty());
    }
}


#[cfg(test)]
mod get_file_content_tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_file_with_content(file_name: &str, content: &str) -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(file_name);

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", content).unwrap();

        (dir, file_path)
    }

    #[test]
    fn test_get_file_content_with_valid_file() {
        let (_temp_dir, file_path) = create_file_with_content("test.txt", "Hello, world!");
        let content = get_file_content(&file_path);
        assert_eq!(content, "Hello, world!\n");
    }
    #[test]
    fn test_get_file_content_with_non_existent_file() {
        let file_path = Path::new("/non/existent/file.txt");
        let content = get_file_content(&file_path);
        assert!(content.is_empty());
    }
}
