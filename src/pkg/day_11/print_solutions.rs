use std::io::BufRead;

use crate::pkg::utils::get_file_reader;

use super::monkey::Monkey;

pub fn print_solutions() {
    let filename = "./src/pkg/day_11/input.txt";

    let monkeys = get_monkeys(filename);
    let simulated = simualte_monkeys(monkeys, 20);
    let monkey_business = get_monkey_business(simulated);

    println!("Day 11:");
    println!(
        "The monkey business after 20 rounds is {}.",
        monkey_business
    )
}

fn get_monkeys(filename: &str) -> Vec<Monkey> {
    let monkey_lines = group_monkeys(filename);

    return monkey_lines.iter().map(|l| Monkey::from(l)).collect();
}

fn group_monkeys(filename: &str) -> Vec<Vec<String>> {
    let reader = get_file_reader(filename);

    let mut monkey_lines: Vec<Vec<String>> = vec![];
    let mut next_monkey: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim() == "" {
            monkey_lines.push(next_monkey);
            next_monkey = vec![];
        } else {
            next_monkey.push(line);
        }
    }

    if next_monkey.len() != 0 {
        monkey_lines.push(next_monkey)
    }

    return monkey_lines;
}

fn simualte_monkeys(monkeys: Vec<Monkey>, rounds: usize) -> Vec<Monkey> {
    let mut monkeys = monkeys;

    for _ in 0..rounds {
        monkeys = run_round(monkeys);
    }

    return monkeys;
}

fn run_round(monkeys: Vec<Monkey>) -> Vec<Monkey> {
    let mut next_monkeys = monkeys.clone();

    for i in 0..next_monkeys.len() {
        while next_monkeys[i].has_items() {
            let item = next_monkeys[i].pop_item();

            let item = next_monkeys[i].perform_operation(item);
            let target = next_monkeys[i].determine_target(item);

            next_monkeys[target].push_item(item);
        }
    }

    return next_monkeys;
}

fn get_monkey_business(monkeys: Vec<Monkey>) -> usize {
    let mut scores: Vec<usize> =
        monkeys.iter().map(|m| m.num_inspections).collect();

    scores.sort();

    return scores[scores.len() - 1] * scores[scores.len() - 2];
}
