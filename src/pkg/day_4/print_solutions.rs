use crate::pkg::utils::read_to_struct;

use super::range_pair::RangePair;

pub fn print_solutions() {
    let filename = "./src/pkg/day_4/input.txt";

    let pairs: Vec<RangePair> = read_to_struct(filename);

    let part_1_answer = get_num_pairs_where_one_contains_other(&pairs);
    let part_2_answer = get_num_pairs_with_any_overlap(&pairs);

    println!("Day 4:");
    println!("The number of pairs of elves whose ranges form a subset is {part_1_answer}.");
    println!("The number of pairs of elves whose ranges have any overlap is {part_2_answer}.");
}

fn get_num_pairs_where_one_contains_other(pairs: &Vec<RangePair>) -> usize {
    return pairs.iter().filter(|p| p.does_one_contain_other()).count();
}

fn get_num_pairs_with_any_overlap(pairs: &Vec<RangePair>) -> usize {
    return pairs.iter().filter(|p| p.is_there_any_overlap()).count();
}
