use std::{cmp, io::BufRead, rc::Rc};

use crate::pkg::utils::get_file_reader;

use super::{graph::Graph, node::Node};

pub fn print_solutions() {
    let filename = "./src/pkg/day_12/input.txt";

    let graph = read_graph(filename);
    let shortest_path =
        graph.find_shortest_path(graph.get_start()).unwrap().len();
    let shortest_with_any_start =
        get_shortest_path_with_any_starting_point(&graph);

    println!("Day 12:");
    println!("The shortest path is of length {}.", shortest_path);
    println!(
        "The shortest path starting from any point with \"a\" height is {}.",
        shortest_with_any_start
    );
}

fn read_graph(filename: &str) -> Graph {
    let reader = get_file_reader(filename);

    let mut rows: Vec<Vec<Node>> = vec![];

    for (row_num, line) in reader.lines().enumerate() {
        let line = &line.unwrap();
        let mut row: Vec<Node> = vec![];

        for (col_num, height) in line.chars().enumerate() {
            row.push(Node::new(height, row_num, col_num));
        }

        rows.push(row);
    }

    return Graph::new(rows);
}

fn get_shortest_path_with_any_starting_point(graph: &Graph) -> usize {
    let starts = graph.get_zero_heights();
    let paths = starts
        .iter()
        .map(|s| graph.find_shortest_path(Rc::clone(s)));
    let valid_paths = paths.filter(|p| p.is_some());
    let lengths = valid_paths.map(|p| p.unwrap().len());
    return lengths.fold(usize::MAX, |shortest, l| cmp::min(shortest, l));
}
