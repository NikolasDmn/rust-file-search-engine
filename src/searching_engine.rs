use crate::file_entry::FileEntry; 
use crate::file_logic::*;
use std::{path::Path, collections::{HashMap, HashSet}, hash::Hash, fs::File};

pub(crate) fn add_entry(file: &FileEntry, indexer: &mut HashMap<String, HashSet<usize>>){
    let file_data = file.title.clone() + " " + &file.content;
    let words = file_data.split(" ");
    for word in words {
        if !indexer.contains_key(word) {
            indexer.insert(word.to_string(), HashSet::new());
        }
        if let Some(entry) = indexer.get_mut(word) {
            entry.insert(file.id);
        }
    }
}
pub(crate) fn query(files: &HashMap<usize, FileEntry>, indexer: &HashMap<String, HashSet<usize>>, query: Vec<String>) -> Vec<(usize, f32)>{
    let mut result: HashMap<usize, f32> = HashMap::new();
    
    for key in query{
        if !indexer.contains_key(&key) {
            continue;
        }
        let mut ids = indexer.get(&key).unwrap();
        let idf = (files.len() as f32 / ids.len() as f32).log10();
        for id in ids {
            let term_frequency = term_frequency(files.get(id).unwrap(), &key);
            if !result.contains_key(id) {
                result.insert(*id, 0.0);
            }
            if let Some(entry) = result.get_mut(id) {
                *entry += term_frequency as f32 * idf ;
            }
        }
    }let mut sorted_entries: Vec<_> = result.into_iter().collect();
    sorted_entries.sort_by(|a, b| a.partial_cmp(b).unwrap());
    return sorted_entries;
}
fn term_frequency(file: &FileEntry, query: &String) -> usize {
    let doc = file.title.to_string() + " " + &file.content;
    return doc.matches(query).count();
}