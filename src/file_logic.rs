use std::path::Path;
use std::fs;

// Get the files in a directory
pub fn get_files(path: &Path) -> Vec<String> {
    let mut files = vec![];
    let entries = std::fs::read_dir(path).unwrap();
    for entry in entries.flatten() {
        let file_type = entry.file_type().ok();
        if file_type.map(|ft| ft.is_dir()).unwrap_or(false) {
            files.extend(get_files(&entry.path()));
        } else if file_type.map(|ft| ft.is_file()).unwrap_or(false) {
            if let Some(file_name) = entry.path().to_str() {
                files.push(file_name.to_owned());
            }
        }
    }
    return files;
}
// Get the content of a file
pub fn get_file_content(path: &Path) -> String {
    let file_content = fs::read_to_string(path);
    return file_content.unwrap_or(String::from(""));
}

// Get the title of a file
pub fn get_file_title(path: &Path) -> String {
    let file_title = path.file_stem().and_then(|stem| stem.to_str());
    return file_title.unwrap_or(&String::from("")).to_string();
}
