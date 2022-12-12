use std::{io::BufRead, rc::Rc};

use crate::pkg::utils::get_file_reader;

use super::{cd::Cd, dir::Dir, ls::Ls};

pub fn print_solutions() {
    let filename = "./src/pkg/day_7/test.txt";

    println!("Day 7:");
}

fn read_file_system(filename: &str) -> &Dir {
    let mut working_directory: Rc<Dir> =
        Rc::new(Dir::new("/".to_string(), None));

    let commands = group_by_commands(filename);

    for command in commands {
        let line = &command[0];

        if is_cd(&line) {
            let cd = Cd::from(command);
            working_directory = Dir::cd(Rc::clone(&working_directory), cd);
        } else if is_ls(&line) {
            let ls = Ls::from(command);
            Dir::add_ls(Rc::clone(&working_directory), ls);
        }
    }

    return working_directory.get_root();
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

fn is_dir(line: &str) -> bool {
    return line.split(" ").nth(0).unwrap() != "dir";
}

fn is_file(line: &str) -> bool {
    return !is_command(line) && !is_dir(line);
}
