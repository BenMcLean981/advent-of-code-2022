use std::cmp;

use super::xy::Xy;

#[derive(Clone)]
pub(crate) struct Bounds {
    pub min_x: usize,
    pub min_y: usize,
    pub max_x: usize,
    pub max_y: usize,
}

impl Bounds {
    pub fn new(
        min_x: usize,
        min_y: usize,
        max_x: usize,
        max_y: usize,
    ) -> Self {
        return Bounds {
            min_x,
            min_y,
            max_x,
            max_y,
        };
    }

    pub fn from_points(points: &Vec<Xy>) -> Self {
        let mut min_x = points[0].x;
        let mut min_y = points[0].y;

        let mut max_x = points[0].x;
        let mut max_y = points[0].y;

        for p in points {
            min_x = cmp::min(min_x, p.x);
            min_y = cmp::min(min_y, p.y);

            max_x = cmp::max(max_x, p.x);
            max_y = cmp::max(max_y, p.y);
        }

        return Bounds::new(min_x, min_y, max_x, max_y);
    }

    pub fn union(bounds: Vec<Bounds>) -> Self {
        let mut min_x = bounds[0].min_x;
        let mut min_y = bounds[0].min_y;

        let mut max_x = bounds[0].max_x;
        let mut max_y = bounds[0].max_y;

        for b in bounds {
            min_x = cmp::min(min_x, b.min_x);
            min_y = cmp::min(min_y, b.min_y);

            max_x = cmp::max(max_x, b.max_x);
            max_y = cmp::max(max_y, b.max_y);
        }

        return Bounds::new(min_x, min_y, max_x, max_y);
    }

    pub fn contains_x(&self, x: usize) -> bool {
        return self.min_x <= x && x <= self.max_x;
    }

    pub fn contains_y(&self, y: usize) -> bool {
        return self.min_y <= y && y <= self.max_y;
    }

    pub fn get_x_range(&self) -> usize {
        return self.max_x - self.min_x;
    }

    pub fn get_y_range(&self) -> usize {
        return self.max_y - self.min_y;
    }
}
