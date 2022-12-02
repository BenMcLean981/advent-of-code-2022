use std::fs::File;
use std::io::BufReader;

pub fn get_file_reader(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("File could not be opened.");
    let reader = BufReader::new(file);
    reader
}
