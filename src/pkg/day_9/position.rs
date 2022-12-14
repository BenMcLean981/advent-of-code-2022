use std::cmp;

use super::direction::Direction;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub(crate) struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub(crate) fn new(x: isize, y: isize) -> Self {
        return Position { x, y };
    }

    pub(crate) fn apply_move(&self, d: Direction) -> Position {
        match d {
            Direction::Up => Position::new(self.x, self.y + 1),
            Direction::Down => Position::new(self.x, self.y - 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }

    pub(crate) fn get_distance(&self, other: Position) -> usize {
        let dx = (self.x - other.x).abs_diff(0);
        let dy = (self.y - other.y).abs_diff(0);

        return cmp::max(dx, dy);
    }
}
