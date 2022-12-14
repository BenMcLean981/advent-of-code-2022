use super::{direction::Direction, motion::Motion, position::Position};

#[derive(Clone)]
pub(crate) struct Rope {
    pub knots: Vec<Position>,
}

impl Rope {
    pub(crate) fn new(knots: Vec<Position>) -> Rope {
        return Rope { knots };
    }

    pub(crate) fn zero(num_knots: usize) -> Rope {
        let mut knots: Vec<Position> = vec![];

        for _ in 0..num_knots {
            knots.push(Position::zero());
        }

        return Rope::new(knots);
    }

    /* Returns each step in the move, excluding the current state. */
    pub(crate) fn simulate_motion(&self, m: &Motion) -> Vec<Rope> {
        let mut steps: Vec<Rope> = vec![];

        for _ in 0..m.distance {
            let current_state = steps.last().unwrap_or(self);
            let moved = current_state.apply_move(m.direction);

            steps.push(moved);
        }

        return steps;
    }

    /*
     * This is definitely not the nicest code but
     * I can't think of a better way to do this
     * right now without overcomplicating things.
     */
    fn apply_move(&self, d: Direction) -> Rope {
        let moved_head = self.knots[0].apply_move(d);
        let mut new_knot_positions: Vec<Position> = vec![moved_head];

        for (i, knot) in self.knots[1..].iter().enumerate() {
            let ahead = new_knot_positions[i];

            let required_dir = knot.get_direction_to_be_near_head(ahead);

            let new_knot_position = match required_dir {
                Some(d) => knot.apply_move(d),
                None => *knot,
            };

            new_knot_positions.push(new_knot_position);
        }

        return Rope::new(new_knot_positions);
    }

    pub(crate) fn get_tail(&self) -> Position {
        return self.knots[self.knots.len() - 1];
    }
}
