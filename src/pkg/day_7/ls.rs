use std::str::FromStr;

use super::file::File;

pub(crate) struct Ls {
    pub(crate) files: Vec<File>,
    pub(crate) dirs: Vec<String>,
}

impl From<Vec<String>> for Ls {
    fn from(command: Vec<String>) -> Self {
        let lines: Vec<String> = command[1..].to_vec();

        let mut files: Vec<File> = vec![];
        let mut dirs: Vec<String> = vec![];

        for line in lines {
            match File::from_str(line.as_str()) {
                Ok(f) => files.push(f),
                Err(_) => dirs
                    .push(line.split(" ").collect::<Vec<_>>()[1].to_string()),
            }
        }

        return Ls { files, dirs };
    }
}
