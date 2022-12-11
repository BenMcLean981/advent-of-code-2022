use std::io::BufRead;
use std::str::FromStr;

use crate::pkg::utils::get_file_reader;

use super::{move_order::MoveOrder, workspace::Workspace};

pub(crate) fn read_input(filename: &str) -> (Workspace, Vec<MoveOrder>) {
    let split = split_file_conents(filename);

    return (read_workspace(split.0), read_orders(split.1));
}

fn split_file_conents(filename: &str) -> (Vec<String>, Vec<String>) {
    let mut workspace: Vec<String> = vec![];
    let mut orders: Vec<String> = vec![];

    let mut past_workspace: bool = false;

    let reader = get_file_reader(filename);
    for line in reader.lines() {
        let line: String = line.unwrap();

        if line.trim() == "" {
            past_workspace = true;
        } else if past_workspace {
            orders.push(line);
        } else {
            workspace.push(line);
        }
    }

    return (workspace, orders);
}

fn read_workspace(mut lines: Vec<String>) -> Workspace {
    lines.reverse();
    let workspace_lines = &lines[1..];

    let len = workspace_lines[0].len();
    let size = line_idx_to_stack_idx(len);

    let mut workspace: Workspace = Workspace::make_of_size(size);

    for line in workspace_lines {
        for idx in (1..len).step_by(4) {
            let c: char = line.chars().nth(idx).unwrap();
            if c != ' ' {
                let stack_idx = line_idx_to_stack_idx(idx);
                workspace.push_to(stack_idx, c);
            }
        }
    }

    return workspace;
}

fn line_idx_to_stack_idx(idx: usize) -> usize {
    return (idx + 1) / 4;
}

fn read_orders(lines: Vec<String>) -> Vec<MoveOrder> {
    let mut orders: Vec<MoveOrder> = vec![];

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();

        let quantity = usize::from_str(split[1]).unwrap();
        let from = usize::from_str(split[3]).unwrap();
        let to = usize::from_str(split[5]).unwrap();

        let order = MoveOrder::new(quantity, from, to);
        orders.push(order);
    }

    return orders;
}
