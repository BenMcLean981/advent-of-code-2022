use std::{fmt, str::FromStr};

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Xy {
    pub x: usize,
    pub y: usize,
}

impl Xy {
    pub fn new(x: usize, y: usize) -> Self {
        return Xy { x, y };
    }
}

#[derive(Debug)]
pub struct PointParseError;

impl fmt::Display for PointParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Could not read Point.");
    }
}

impl FromStr for Xy {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(",").collect();

        let x = usize::from_str(split[0]).unwrap();
        let y = usize::from_str(split[1]).unwrap();

        return Ok(Xy::new(x, y));
    }
}
