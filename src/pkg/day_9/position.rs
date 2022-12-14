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
            Direction::UpRight => Position::new(self.x + 1, self.y + 1),
            Direction::UpLeft => Position::new(self.x - 1, self.y + 1),
            Direction::DownRight => Position::new(self.x + 1, self.y - 1),
            Direction::DownLeft => Position::new(self.x - 1, self.y - 1),
        }
    }

    /* I really cannot think of a nice way to do this other than enum matching. */
    pub(crate) fn get_direction_to_be_near_head(
        &self,
        ahead: Position,
    ) -> Option<Direction> {
        let d_x: isize = ahead.x - self.x;
        let d_y: isize = ahead.y - self.y;

        if is_up(d_x, d_y) {
            return Some(Direction::Up);
        }
        if is_down(d_x, d_y) {
            return Some(Direction::Down);
        }
        if is_left(d_x, d_y) {
            return Some(Direction::Left);
        }
        if is_right(d_x, d_y) {
            return Some(Direction::Right);
        }
        if is_up_left(d_x, d_y) {
            return Some(Direction::UpLeft);
        }
        if is_up_right(d_x, d_y) {
            return Some(Direction::UpRight);
        }
        if is_down_left(d_x, d_y) {
            return Some(Direction::DownLeft);
        }
        if is_down_right(d_x, d_y) {
            return Some(Direction::DownRight);
        } else {
            return None;
        }
    }

    pub(crate) fn zero() -> Position {
        return Position::new(0, 0);
    }
}

fn is_up(d_x: isize, d_y: isize) -> bool {
    return d_x == 0 && d_y == 2;
}

fn is_right(d_x: isize, d_y: isize) -> bool {
    return d_x == 2 && d_y == 0;
}

fn is_down(d_x: isize, d_y: isize) -> bool {
    return d_x == 0 && d_y == -2;
}

fn is_left(d_x: isize, d_y: isize) -> bool {
    return d_x == -2 && d_y == 0;
}

fn is_up_right(d_x: isize, d_y: isize) -> bool {
    return d_x == 1 && d_y == 2
        || d_x == 2 && d_y == 2
        || d_x == 2 && d_y == 1;
}

fn is_up_left(d_x: isize, d_y: isize) -> bool {
    return d_x == -2 && d_y == 1
        || d_x == -2 && d_y == 2
        || d_x == -1 && d_y == 2;
}

fn is_down_right(d_x: isize, d_y: isize) -> bool {
    return d_x == 2 && d_y == -1
        || d_x == 2 && d_y == -2
        || d_x == 1 && d_y == -2;
}

fn is_down_left(d_x: isize, d_y: isize) -> bool {
    return d_x == -1 && d_y == -2
        || d_x == -2 && d_y == -2
        || d_x == -2 && d_y == -1;
}
