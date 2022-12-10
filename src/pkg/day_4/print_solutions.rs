use crate::pkg::utils::read_to_struct;

use super::range_pair::RangePair;

pub fn print_solutions() {
    let filename = "./src/pkg/day_4/input.txt";

    let pairs: Vec<RangePair> = read_to_struct(filename);

    let part_1_answer = get_num_pairs_with_subset(pairs);

    println!("Day 4:");
    println!("The number of pairs of elves whose ranges form a subset is {part_1_answer}.")
}

fn get_num_pairs_with_subset(pairs: Vec<RangePair>) -> usize {
    return pairs
        .iter()
        .filter(|p| p.does_one_contain_other())
        .count();
}
