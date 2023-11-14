mod file_logic;
mod file_entry;
mod searching_engine;
use file_entry::FileEntry;
use std::{path::Path, collections::{HashMap}, env};
use std::io;
use std::io::{ Write};

fn main() {
    // Parse the first command-line argument as the directory path
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        return;
    }
    let dir_path = Path::new(&args[1]);

    // Process the directory and generate an indexer
    println!("Fetching files...");
    let files = file_logic::get_file_set(dir_path);
    println!("Fetched files.");
    println!("Indexing files...");
    let indexer = searching_engine::generate_indexer(&files);
    println!("Indexed files.");

    loop {
        let query = get_query();
        if query.iter().any(|q| q == "\\exit") {
            break;
        }

        let search_results = searching_engine::query(&files, &indexer, query);
        display_results(&search_results,&files); // Implement this function to display results nicely
    }
}
fn get_query() -> Vec<String> {
    print!("Enter query (or \\exit to close): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.split_whitespace().map(String::from).collect();
}
fn display_results(results: &Vec<(usize, f32)>, files: &HashMap<usize, FileEntry>) {
    for (index, (id, _score)) in results.iter().enumerate() {
        println!("{}: {}", index+1, files.get(id).unwrap().path.to_str().unwrap());
    }
}