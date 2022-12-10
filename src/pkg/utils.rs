use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn get_file_reader(filename: &str) -> BufReader<File> {
    let file =
        File::open(filename).expect("File could not be opened.");
    let reader = BufReader::new(file);
    reader
}

pub fn read_to_struct<T: FromStr>(filename: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let reader = get_file_reader(filename);

    let mut results: Vec<T> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        results.push(T::from_str(&line).unwrap());
    }

    return results;
}
