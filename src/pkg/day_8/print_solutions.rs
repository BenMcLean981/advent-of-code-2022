use std::io::BufRead;

use crate::pkg::utils::get_file_reader;

use super::{grid::Grid, tree::Tree};

pub fn print_solutions() {
    let filename = "./src/pkg/day_8/input.txt";

    println!("Day 8:");

    let grid = read_grid(filename);
    let num_visible = grid.get_all_visible().len();

    println!(
        "The number of visible trees in the grid is: {}.",
        num_visible,
    );
}

fn read_grid(filename: &str) -> Grid {
    let reader = get_file_reader(filename);

    let mut rows: Vec<Vec<Tree>> = vec![];

    for (y_pos, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut row: Vec<Tree> = vec![];

        for (x_pos, c) in line.chars().enumerate() {
            let tree = Tree::new(
                c.to_digit(10).unwrap().try_into().unwrap(),
                x_pos,
                y_pos,
            );
            row.push(tree);
        }

        rows.push(row);
    }

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    return Grid::new(rows, num_rows, num_cols);
}
