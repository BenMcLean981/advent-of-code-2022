#[derive(Copy, Clone)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => {
                panic!("The character='{}' does not represent a direction.", c)
            }
        }
    }
}
