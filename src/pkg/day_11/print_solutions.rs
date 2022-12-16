use std::io::BufRead;

use crate::pkg::utils::get_file_reader;

use super::monkey::Monkey;

pub fn print_solutions() {
    let filename = "./src/pkg/day_11/input.txt";

    let monkeys = get_monkeys(filename);

    let destressing_rounds = 20;
    let destressing_simulation =
        simualte_monkeys(monkeys.clone(), destressing_rounds, &|x| x / 3);
    let destressed_monkey_business =
        get_monkey_business(destressing_simulation);

    let stressing_rounds = 10_000;
    let divisor = lcm(&monkeys);
    let stressing_simulation =
        simualte_monkeys(monkeys, stressing_rounds, &|x| x % divisor);
    let stressed_monkey_business = get_monkey_business(stressing_simulation);

    println!("Day 11:");
    println!(
        "The monkey business after {} destressing rounds is {}.",
        destressing_rounds, destressed_monkey_business
    );
    println!(
        "The monkey business after {} stressing rounds is {}.",
        stressing_rounds, stressed_monkey_business
    );
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

fn simualte_monkeys(
    monkeys: Vec<Monkey>,
    rounds: usize,
    reducer: &dyn Fn(usize) -> usize,
) -> Vec<Monkey> {
    let mut monkeys = monkeys;

    for _ in 0..rounds {
        monkeys = run_round(monkeys, reducer);
    }

    return monkeys;
}

fn run_round(
    monkeys: Vec<Monkey>,
    reducer: &dyn Fn(usize) -> usize,
) -> Vec<Monkey> {
    let mut next_monkeys = monkeys.clone();

    for i in 0..next_monkeys.len() {
        while next_monkeys[i].has_items() {
            let item = next_monkeys[i].pop_item();

            let item = next_monkeys[i].perform_operation(item, reducer);
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

fn lcm(monkeys: &Vec<Monkey>) -> usize {
    return monkeys
        .iter()
        .fold(1, |prod, m| prod * m.divisor / gcd(prod, m.divisor));
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    } else {
        return gcd(b % a, a);
    }
}
