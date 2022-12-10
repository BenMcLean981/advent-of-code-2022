use std::{io::BufRead, str::FromStr};

use crate::pkg::utils::{get_file_reader, read_to_struct};

use super::{elf_group::ElfGroup, rucksack::Rucksack};

pub fn print_solutions() {
    let filename = "./src/pkg/day_3/input.txt";

    let part_1_sum = get_part_1_sum(filename);
    let part_2_sum = get_part_2_sum(filename);

    println!("Day 3:");
    println!("The sum of common items in each rucksack is {part_1_sum}.");
    println!("The sum of elf group badge priorities is {part_2_sum}.");
}

fn get_part_1_sum(filename: &str) -> u32 {
    let rucksacks: Vec<Rucksack> = read_to_struct(filename);
    let sum = rucksacks.iter().fold(0, |sum, rucksack| {
        sum + rucksack.get_priority_of_common_item()
    });

    return sum;
}

fn get_part_2_sum(filename: &str) -> u32 {
    let elf_groups: Vec<ElfGroup> = parse_elf_groups(filename);
    let sum = elf_groups
        .iter()
        .fold(0, |sum, elf_group| sum + elf_group.get_badge_pririty());

    return sum;
}

fn parse_elf_groups(filename: &str) -> Vec<ElfGroup> {
    let reader = get_file_reader(filename);

    let mut results: Vec<ElfGroup> = vec![];

    let mut collector: Vec<Rucksack> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        collector.push(Rucksack::from_str(&line).unwrap());

        if collector.len() == 3 {
            let next_group: ElfGroup = ElfGroup::new(
                collector[0].clone(),
                collector[1].clone(),
                collector[2].clone(),
            );
            results.push(next_group);

            collector.clear();
        }
    }

    return results;
}
