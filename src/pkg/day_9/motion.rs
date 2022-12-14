use std::{fmt, str::FromStr};

use super::direction::Direction;

pub(crate) struct Motion {
    pub direction: Direction,
    pub distance: usize,
}

impl Motion {
    pub(crate) fn new(direction: Direction, distance: usize) -> Self {
        return Motion {
            direction,
            distance,
        };
    }
}

#[derive(Debug)]
pub(crate) struct MotionParseError;

impl fmt::Display for MotionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Could not read Motion.");
    }
}

impl FromStr for Motion {
    type Err = MotionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.chars().nth(0).unwrap();
        let direction = Direction::from(direction);

        let distance = &s[2..];
        let distance = usize::from_str(distance).unwrap();

        return Ok(Motion::new(direction, distance));
    }
}
