use crate::file_entry::FileEntry; 
use std::{ collections::{HashMap, HashSet}};

pub(crate) fn generate_indexer(files: &HashMap<usize, FileEntry>) -> HashMap<String, HashSet<usize>> {
    let mut indexer = HashMap::new();
    // Add each file's data to the indexer
    files.values().for_each(|file| add_entry(file, &mut indexer));
    indexer
}



pub(crate) fn add_entry(file: &FileEntry, indexer: &mut HashMap<String, HashSet<usize>>) {
    // Combine title and content, then iterate over each word
    let file_data = file.title.clone() + " " + &file.content;
    for word in file_data.split_whitespace() {
        indexer.entry(word.to_string()).or_insert_with(HashSet::new).insert(file.id);
    }
}

pub(crate) fn query(files: &HashMap<usize, FileEntry>, indexer: &HashMap<String, HashSet<usize>>, query: Vec<String>) -> Vec<(usize, f32)> {
    let mut result = HashMap::new();

    for key in query {
        if let Some(ids) = indexer.get(&key) {
            let idf = (files.len() as f32 / ids.len() as f32).log10();  // Calculate IDF

            for id in ids {
                let term_frequency = term_frequency(files.get(id).unwrap(), &key);
                // Calculate TF-IDF and add to result
                *result.entry(*id).or_insert(0.0) += term_frequency as f32 * idf;
            }
        }
    }

    // Sort the results by score in descending order
    let mut sorted_entries: Vec<_> = result.into_iter().collect();
    sorted_entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    sorted_entries
}

fn term_frequency(file: &FileEntry, query: &String) -> usize {
    let doc = file.title.to_string() + " " + &file.content;
    return doc.matches(query).count();
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::collections::{HashMap};

    // Mock data for testing
    fn create_mock_files() -> HashMap<usize, FileEntry> {
        let mut files = HashMap::new();
        files.insert(1, FileEntry {
            id: 1,
            title: "file1".to_string(),
            content: "Content of file one".to_string(),
            path: PathBuf::from("/path/to/file1"),
        });
        files.insert(2, FileEntry {
            id: 2,
            title: "file2".to_string(),
            content: "Content of file two".to_string(),
            path: PathBuf::from("/path/to/file2"),
        });
        files
    }

    #[test]
    fn test_generate_indexer() {
        let files = create_mock_files();
        let indexer = generate_indexer(&files);
        assert!(indexer.contains_key("Content"));
    }

    #[test]
    fn test_add_entry() {
        let mut indexer = HashMap::new();
        let file = FileEntry {
            id: 1,
            title: "test".to_string(),
            content: "This is a test file".to_string(),
            path: PathBuf::from("/path/to/test"),
        };
        add_entry(&file, &mut indexer);
        assert!(indexer.contains_key("test"));
    }

    #[test]
    fn test_query() {
        let files = create_mock_files();
        let indexer = generate_indexer(&files);
        let results = query(&files, &indexer, vec!["file".to_string()]);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_term_frequency() {
        let file = FileEntry {
            id: 1,
            title: "test".to_string(),
            content: "test test test".to_string(),
            path: PathBuf::from("/path/to/test"),
        };
        let frequency = term_frequency(&file, &"test".to_string());
        assert_eq!(frequency, 4);
    }
}