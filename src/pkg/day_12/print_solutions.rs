use std::io::BufRead;

use crate::pkg::{day_12::node::Height, utils::get_file_reader};

use super::{graph::Graph, node::Node};

pub fn print_solutions() {
    let filename = "./src/pkg/day_12/input.txt";

    let graph = read_graph(filename);
    let shortest_path = graph.find_shortest_path(Height::Start).len();
    let shortest_with_any_start =
        graph.find_shortest_path(Height::NodeHeight(0)).len();

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
