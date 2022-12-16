use crate::pkg::utils::read_to_struct;

use super::{cpu::Cpu, instructions::instruction::Instruction};

pub fn print_solutions() {
    let filename = "./src/pkg/day_10/input.txt";

    let instructions = get_instructions(filename);
    let cpu_states = simulate_cpu(instructions);

    let signal_strengths = get_signal_strengths(&cpu_states);

    println!("Day 10:");
    println!(
        "The sum of the relevant signal strengths is {}.",
        signal_strengths
    )
}

fn get_instructions(filename: &str) -> Vec<Box<dyn Instruction>> {
    return read_to_struct(filename);
}

fn simulate_cpu(instructions: Vec<Box<dyn Instruction>>) -> Vec<Cpu> {
    let mut states: Vec<Cpu> = vec![Cpu::initialize()];

    for i in instructions {
        let cpu = &states[states.len() - 1];

        let simualted = i.simulate(cpu);
        states.extend_from_slice(&simualted);
    }

    return states;
}

fn get_signal_strengths(cpu_states: &Vec<Cpu>) -> isize {
    let cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];

    return cycles
        .iter()
        .fold(0, |sum, c| sum + get_cycle_signal_strength(cpu_states, *c));
}

fn get_cycle_signal_strength(cpu_states: &Vec<Cpu>, cycle: usize) -> isize {
    let state = &cpu_states[cycle - 1];

    return state.x * isize::try_from(cycle).unwrap();
}
