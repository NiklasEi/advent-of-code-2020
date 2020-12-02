use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_to_vec(path: &str) -> Vec<String> {
    let mut entries = vec![];
    let file = File::open(path).expect("Unable to open file");
    for line_result in BufReader::new(file).lines() {
        if let Ok(line) = line_result {
            entries.push(line);
        }
    }
    entries
}
