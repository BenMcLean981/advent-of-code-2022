use std::{io::BufRead, str::FromStr};

use crate::pkg::utils::get_file_reader;

use super::{game::Game, shape::Shape, winner::Winner};

pub fn print_solutions() {
    let filename = "./src/pkg/day_2/input.txt";

    let part_1_games = parse_part_1_games(filename);
    let part_1_score = get_total_score(part_1_games);

    let part_2_games: Vec<Game> = parse_part_2_games(filename);
    let part_2_score = get_total_score(part_2_games);

    println!("Day 2:");
    println!("The total score for day 2 part one is {}.", part_1_score);
    println!("The total score for day 2 part two is {}.", part_2_score);
}

fn parse_part_1_games(filename: &str) -> Vec<Game> {
    let reader = get_file_reader(filename);

    let mut results: Vec<Game> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        let their_play: Shape = Shape::from_str(&line[0..1]).unwrap();
        let my_play: Shape = Shape::from_str(&line[2..3]).unwrap();

        let game = Game::new_part_1(my_play, their_play);
        results.push(game);
    }

    return results;
}

fn parse_part_2_games(filename: &str) -> Vec<Game> {
    let reader = get_file_reader(filename);

    let mut results: Vec<Game> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        let their_play: Shape = Shape::from_str(&line[0..1]).unwrap();
        let winner: Winner = Winner::from_str(&line[2..3]).unwrap();

        let game = Game::new_part_2(their_play, winner);
        results.push(game);
    }

    return results;
}

fn get_total_score(games: Vec<Game>) -> u32 {
    return games.iter().fold(0, |sum, g| sum + g.get_score());
}
