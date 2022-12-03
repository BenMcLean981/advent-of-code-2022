use std::{io::BufRead, str::FromStr};

use crate::pkg::utils::get_file_reader;

use super::rucksack::Rucksack;

pub fn print_solutions() {
    let filename = "./src/pkg/day_3/input.txt";

    let rucksacks: Vec<Rucksack> = parse_rucksacks(filename);
    let sum = rucksacks.iter().fold(0, |sum, rucksack| {
        sum + rucksack.get_priority_of_common_item()
    });

    println!("Day 3:");
    println!("The sum of common items in each rucksack is {sum}.",)
}

fn parse_rucksacks(filename: &str) -> Vec<Rucksack> {
    let reader = get_file_reader(filename);

    let mut results: Vec<Rucksack> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        results.push(Rucksack::from_str(&line).unwrap());
    }

    return results;
}
