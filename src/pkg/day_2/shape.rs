use std::{fmt, str::FromStr};

#[derive(Debug)]
pub(crate) struct ShapeParseError;

impl fmt::Display for ShapeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Could not read shape.");
    }
}

#[derive(Clone)]
pub(crate) enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ShapeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "Y" => Ok(Shape::Paper),
            "X" => Ok(Shape::Rock),
            "Z" => Ok(Shape::Scissors),
            _ => Err(ShapeParseError),
        }
    }
}
