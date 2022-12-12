use std::io::BufRead;

use crate::pkg::utils::get_file_reader;

use super::{cd::Cd, ls::Ls, workspace::WorkSpace};

pub fn print_solutions() {
    let filename = "./src/pkg/day_7/input.txt";

    let file_system = read_file_system(filename);
    let part_1_size = 100000;
    let sum_under_size = file_system.get_sum_under(part_1_size);

    let disk_size = 70000000;
    let req_size = 30000000;
    let smallest_over_size: usize =
        file_system.get_smallest_over(disk_size, req_size);

    println!("Day 7:");
    println!(
        "The sum of all files of size under {} is {}.",
        part_1_size, sum_under_size
    );
    println!(
        "The smallest that can be deleted to free up to {} is of size {}.",
        req_size, smallest_over_size
    );
}

fn read_file_system(filename: &str) -> WorkSpace {
    let mut workspace = WorkSpace::new();
    let mut working_directory: usize = 0;

    let commands = group_by_commands(filename);

    for command in commands {
        let line = &command[0];

        if is_cd(&line) {
            let cd = Cd::from(command);
            working_directory = workspace.dirs[working_directory].cd(cd);
        } else if is_ls(&line) {
            let ls = Ls::from(command);
            workspace.add_ls(working_directory, ls)
        }
    }

    return workspace.clone();
}

fn group_by_commands(filename: &str) -> Vec<Vec<String>> {
    let reader = get_file_reader(filename);

    let mut commands: Vec<Vec<String>> = vec![];
    let mut next_command: Vec<String> = vec![];

    for line in reader.lines() {
        let line: String = line.unwrap();

        if is_command(&line) {
            if next_command.len() > 0 {
                commands.push(next_command);
                next_command = vec![];
            }
        }
        next_command.push(line);
    }

    commands.push(next_command);

    return commands;
}

fn is_ls(line: &str) -> bool {
    return is_command_type(line, "ls");
}

fn is_cd(line: &str) -> bool {
    return is_command_type(line, "cd");
}

fn is_command_type(line: &str, cmd_type: &str) -> bool {
    return is_command(line)
        && line.split(" ").collect::<Vec<_>>()[1] == cmd_type;
}

fn is_command(line: &str) -> bool {
    return line.chars().nth(0).unwrap() == '$';
}
