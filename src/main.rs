mod file_logic;
mod file_entry;
mod searching_engine;
use file_entry::FileEntry;
use std::{path::Path, collections::{HashMap, HashSet}, str::EncodeUtf16};
fn main() {
    let files = file_logic::get_files(Path::new("./"));
    let mut indexer: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut fileSet: HashMap<usize, FileEntry> = HashMap::new();
    for (id, file) in files.iter().enumerate() {
        let fileEntry = FileEntry::new(id,Path::new(file));
        searching_engine::add_entry(&fileEntry, &mut indexer);
        fileSet.insert(id, fileEntry);
    }
    let query = vec!["hello".to_string(), "world".to_string()];
    let result = searching_engine::query(&fileSet, &indexer, query);
    for (id, score) in result.iter() {
        println!("{}: {}", score, fileSet.get(id).unwrap().title);
    }
    println!("{:?}", result);
}
