use crate::pkg::{
    day_14::{grid::Grid, path::Path},
    utils::read_to_struct,
};

pub fn print_solutions() {
    println!("Day 14:");

    let filename = "./src/pkg/day_14/input.txt";

    let paths: Vec<Path> = read_to_struct(filename);
    let mut grid = Grid::make_from_rock_paths(&paths, false);

    let sand_x = 500;
    let num_sand_with_abyss: usize =
        grid.get_num_sand_points_with_abyss(sand_x);

    let mut grid = Grid::make_from_rock_paths(&paths, true);
    let num_sand_until_clogged =
        grid.get_num_sand_points_without_abyss(sand_x);

    println!(
        "{} Pieces of sand can drop before falling into the abyss.",
        num_sand_with_abyss
    );
    println!(
        "{} Pieces of sand can drop before the ceiling is clogged.",
        num_sand_until_clogged
    );
}
