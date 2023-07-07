use crate::file_logic::*;
use std::path::{Path, PathBuf};


pub struct FileEntry {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub path: PathBuf,
}

impl FileEntry {
    pub fn new(id: usize, path: &Path) -> FileEntry {
        let title = get_file_title(path);
        let content =  get_file_content(path);
        return FileEntry {
            id,
            title,
            content,
            path: path.to_path_buf(),
        };
    }
}