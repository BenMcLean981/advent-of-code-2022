use std::collections::HashSet;

use crate::pkg::utils::read_to_struct;

use super::{motion::Motion, position::Position, rope::Rope};

pub fn print_solutions() {
    let filename = "./src/pkg/day_9/input.txt";

    let motions = get_motions(filename);
    let num_unique_tail_positions = get_num_unique_tail_positions(motions);

    println!("Day 9:");
    println!(
        "The number of unique tail positions is {}.",
        num_unique_tail_positions,
    );
}

fn get_motions(filename: &str) -> Vec<Motion> {
    return read_to_struct(filename);
}

fn get_num_unique_tail_positions(motions: Vec<Motion>) -> usize {
    let steps = simululate_motions(motions);

    let mut set: HashSet<Position> = HashSet::new();

    for step in steps {
        set.insert(step.tail);
    }

    return set.len();
}

fn simululate_motions(motions: Vec<Motion>) -> Vec<Rope> {
    let mut steps: Vec<Rope> = vec![Rope::zero()];

    for m in motions {
        let rope = &steps[steps.len() - 1];
        let intermediate_steps = rope.simulate_motion(&m);

        intermediate_steps.iter().for_each(|s| steps.push(*s));
    }

    return steps;
}
