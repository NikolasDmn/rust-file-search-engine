# Rust File Search Engine
The Rust File Search Engine is a high-performance, command-line tool designed for indexing and searching text within files in a specified directory. Built in Rust, it offers efficient processing, making it ideal for handling large datasets with speed and accuracy.

## Installation
### Prerequisites
Ensure Rust is installed on your system. If not, follow the installation guide on the [official Rust website.
](https://www.rust-lang.org/learn/get-started)
### Setup
1. Clone the Rust File Search Engine repository:
```bash
git clone https://github.com/your-username/rust-file-search-engine.git
```
2. Navigate to the project directory:
```bash
cd rust-file-search-engine
```
3. Compile the project:
```bash
cargo build --release
```
## Usage
After compiling the project, you can run the search engine directly from the binary:
```bash
./target/release/searcher.{extension_of_os} /path/to/your/directory
```
You can additionally run it directly from cargo:
```bash
cargo run --release -- /path/to/your/directory
```
Both will start the application and prompt you to enter search queries. 
<br/>
To exit, type `\exit`.

## Features
- Efficient Indexing: Rapidly indexes all text files in a specified directory.
- Advanced Search: Uses TF-IDF ranking to provide relevant search results.
- User-Friendly: Interactive command-line interface for seamless searching.
- Scalable: Designed to handle large volumes of text data.
## Unit Testing
Run the following command to execute the unit tests and ensure the reliability of the code:
```bash
cargo test
```
These tests validate the functionality of the core components of the search engine. 
<br/>The coverage is everything that can be tested through unit tests using the [tempfile](https://docs.rs/tempfile/latest/tempfile/) and [tempdir](https://docs.rs/tempdir/latest/tempdir/) crates to test directory/file interaction

## Future Improvements
- Ranking Algorithm Enhancements: Implement more sophisticated algorithms like BM25 for better accuracy.
- Semantic Analysis: Integrate NLP tools for semantic understanding and context-aware searching.
- GUI Implementation: Develop a graphical user interface for broader user accessibility.
- Distributed Processing: Adapt the system for distributed environments to handle extremely large datasets.