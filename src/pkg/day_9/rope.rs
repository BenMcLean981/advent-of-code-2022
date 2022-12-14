use super::{direction::Direction, motion::Motion, position::Position};

#[derive(Clone, Copy)]
pub(crate) struct Rope {
    pub head: Position,
    pub tail: Position,
}

impl Rope {
    pub(crate) fn new(head: Position, tail: Position) -> Rope {
        return Rope { head, tail };
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

    fn apply_move(&self, d: Direction) -> Rope {
        let old_head_position: Position = self.head;
        let mut head_moved = self.move_head(d);
        head_moved.rectify_tail(old_head_position);

        return head_moved;
    }

    fn move_head(&self, d: Direction) -> Rope {
        let moved_head = self.head.apply_move(d);
        return Rope::new(moved_head, self.tail);
    }

    fn rectify_tail(&mut self, old_head_position: Position) {
        let distance = self.tail.get_distance(self.head);

        if distance == 2 {
            self.tail = old_head_position;
        }
    }

    pub(crate) fn zero() -> Rope {
        return Rope::new(Position::new(0, 0), Position::new(0, 0));
    }
}
