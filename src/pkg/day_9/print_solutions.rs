use std::collections::HashSet;

use crate::pkg::utils::read_to_struct;

use super::{motion::Motion, position::Position, rope::Rope};

pub fn print_solutions() {
    let filename = "./src/pkg/day_9/input.txt";

    let motions = get_motions(filename);

    let num_knots_part_1 = 2;
    let num_part_1_positions =
        get_num_unique_tail_positions(&motions, num_knots_part_1);

    let num_knots_part_2 = 10;
    let num_part_2_positions =
        get_num_unique_tail_positions(&motions, num_knots_part_2);

    println!("Day 9:");
    println!(
        "The number of unique tail positions with two knots is {}.",
        num_part_1_positions,
    );

    println!(
        "The number of unique tail positions with ten knots is {}.",
        num_part_2_positions,
    );
}

fn get_motions(filename: &str) -> Vec<Motion> {
    return read_to_struct(filename);
}

fn get_num_unique_tail_positions(
    motions: &Vec<Motion>,
    num_knots: usize,
) -> usize {
    let steps = simululate_motions(motions, num_knots);

    let mut set: HashSet<Position> = HashSet::new();

    for step in steps {
        set.insert(step.get_tail());
    }

    return set.len();
}

fn simululate_motions(motions: &Vec<Motion>, num_knots: usize) -> Vec<Rope> {
    let mut steps: Vec<Rope> = vec![Rope::zero(num_knots)];

    for m in motions {
        let rope = &steps[steps.len() - 1];
        let intermediate_steps = rope.simulate_motion(&m);

        intermediate_steps
            .iter()
            .for_each(|s| steps.push(s.clone()));
    }

    return steps;
}
