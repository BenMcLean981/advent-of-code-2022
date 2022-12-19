use super::xy::Xy;

pub(crate) struct Line {
    start: Xy,
    end: Xy,
}

impl Line {
    pub fn new(start: Xy, end: Xy) -> Self {
        return Line { start, end };
    }

    pub fn get_points(&self) -> Vec<Xy> {
        let mut points: Vec<Xy> = vec![];

        if self.is_horizontal() {
            if self.start.x < self.end.x {
                for x in self.start.x..self.end.x + 1 {
                    points.push(Xy::new(x, self.start.y));
                }
            } else {
                for x in self.end.x..self.start.x + 1 {
                    points.push(Xy::new(x, self.start.y));
                }
            }
        } else {
            if self.start.y < self.end.y {
                for y in self.start.y..self.end.y + 1 {
                    points.push(Xy::new(self.start.x, y));
                }
            } else {
                for y in self.end.y..self.start.y + 1 {
                    points.push(Xy::new(self.start.x, y));
                }
            }
        }

        return points;
    }

    fn is_horizontal(&self) -> bool {
        return self.start.y == self.end.y;
    }
}
