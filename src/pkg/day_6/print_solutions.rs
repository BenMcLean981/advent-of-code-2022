use core::panic;
use std::io::BufRead;

use crate::pkg::utils::get_file_reader;

pub fn print_solutions() {
    let filename = "./src/pkg/day_6/input.txt";

    let datastream = read_datastream_buffer(filename);
    let start_of_packet = find_unique_characters(&datastream, 4);
    let start_of_message = find_unique_characters(&datastream, 14);

    println!("Day 6:");
    println!("The datastream start is at character {start_of_packet}.");
    println!("The message start is at character {start_of_message}.");
}

fn read_datastream_buffer(filename: &str) -> String {
    let reader = get_file_reader(filename);
    return reader.lines().nth(0).unwrap().unwrap();
}

fn find_unique_characters(datastream: &String, window_size: usize) -> usize {
    for window_start in 0..(datastream.len() - window_size) {
        let window_end = window_start + window_size;

        let window = &datastream[window_start..window_end];
        let window = window.chars().collect::<Vec<_>>();

        if is_packet_start(&window) {
            return window_end;
        }
    }

    panic!("The given datastream has no packet start.");
}

fn is_packet_start(window: &Vec<char>) -> bool {
    for c1 in window {
        if window.iter().filter(|c2| c1 == *c2).count() != 1 {
            return false;
        }
    }

    return true;
}
