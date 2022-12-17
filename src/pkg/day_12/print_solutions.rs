use std::io::BufRead;

use crate::pkg::utils::get_file_reader;

use super::{graph::Graph, node::Node};

pub fn print_solutions() {
    let filename = "./src/pkg/day_12/input.txt";

    let graph = read_graph(filename);
    let shortest_path = graph.find_shortest_path().len();

    println!("Day 12:");
    println!("The shortest path is of length {}.", shortest_path)
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
