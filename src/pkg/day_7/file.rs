use std::{fmt, str::FromStr};

#[derive(Clone)]
pub(crate) struct File {
    pub size: usize,
    name: String,
}

#[derive(Debug)]
pub(crate) struct FileParseError;

impl fmt::Display for FileParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Could not read file.");
    }
}

impl FromStr for File {
    type Err = FileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ").collect::<Vec<_>>();

        let size = usize::from_str(split[0]);

        match size {
            Ok(size) => Ok(File {
                size,
                name: split[1].to_string(),
            }),
            Err(_) => Err(FileParseError),
        }
    }
}
