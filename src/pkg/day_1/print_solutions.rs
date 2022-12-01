use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solutions() {
    let elves = read_elves("./src/pkg/day_1/input.txt");

    let top_elves: Vec<Elf> = find_top_elves(elves, 3);

    println!("Day 1:");
    print_top_elf(&top_elves);
    print_sum_of_top_elves(&top_elves);
}

fn read_elves(filename: &str) -> Vec<Elf> {
    let reader = get_file_reader(filename);

    let mut results: Vec<Elf> = vec![];
    let mut next_elf: Elf = Elf::new(0);

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            results.push(next_elf.clone());
            next_elf = Elf::new(next_elf.id + 1);
        } else {
            let calories: u32 = line.parse().unwrap();
            next_elf.add_food(calories);
        }
    }

    return results;
}

fn get_file_reader(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("File could not be opened.");
    let reader = BufReader::new(file);
    reader
}

// An alternative here would be to sort.
// However, we know we only need the top n elves (3 in this case)
// there could be thousands of elves, so sorting could be wasteful.
fn find_top_elves(elves: Vec<Elf>, num: u32) -> Vec<Elf> {
    let mut top_elves: Vec<Elf> = vec![];

    for _ in 0..num {
        top_elves.push(find_elf_with_most_calories(&elves, &top_elves));
    }

    return top_elves;
}

fn find_elf_with_most_calories(elves: &Vec<Elf>, exclude_elves: &Vec<Elf>) -> Elf {
    let not_excluded = elves.iter().filter(|e1| !exclude_elves.contains(e1));

    return not_excluded.fold(elves[0].clone(), |max, e| {
        if e.get_total_calories() > max.get_total_calories() {
            return e.clone();
        } else {
            return max;
        }
    });
}

fn print_top_elf(top_elves: &Vec<Elf>) {
    println!(
        "The elf with the most calories has {} calories.",
        top_elves[0].get_total_calories()
    )
}

fn print_sum_of_top_elves(top_elves: &Vec<Elf>) {
    let calories = top_elves
        .iter()
        .fold(0, |sum, e| sum + e.get_total_calories());

    println!(
        "The top {} elves are carrying {} calories.",
        top_elves.len(),
        calories
    )
}

#[derive(Clone, PartialEq)]
struct Elf {
    id: u32,
    food_calories: Vec<u32>,
}

impl Elf {
    fn new(id: u32) -> Self {
        Self {
            id,
            food_calories: vec![],
        }
    }

    pub fn add_food(&mut self, calories: u32) {
        self.food_calories.push(calories);
    }

    pub fn get_total_calories(&self) -> u32 {
        let iter = self.food_calories.iter();
        return iter.fold(0, |sum, calories| calories + sum);
    }
}
