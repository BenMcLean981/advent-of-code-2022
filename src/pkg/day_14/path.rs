use std::{collections::HashSet, fmt, str::FromStr};

use super::{bounds::Bounds, line::Line, xy::Xy};

#[derive(Clone)]
pub(crate) struct Path {
    points: Vec<Xy>,
}

impl Path {
    pub fn new(points: Vec<Xy>) -> Self {
        return Path { points };
    }
}

#[derive(Debug)]
pub struct PathParseError;

impl fmt::Display for PathParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Could not read Path.");
    }
}

impl FromStr for Path {
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let no_space = s.replace(" ", "");
        let point_strs: Vec<&str> = no_space.split("->").collect();
        let points: Vec<Xy> = point_strs
            .iter()
            .map(|s| Xy::from_str(s).unwrap())
            .collect();

        return Ok(Path::new(points));
    }
}

impl Path {
    pub(crate) fn get_rock_points(&self) -> Vec<Xy> {
        let mut points: HashSet<Xy> = HashSet::new();

        for line in self.get_lines() {
            for p in line.get_points() {
                points.insert(p);
            }
        }

        return points.iter().map(|p| *p).collect();
    }

    fn get_lines(&self) -> Vec<Line> {
        let mut lines: Vec<Line> = vec![];

        for i1 in 0..self.points.len() - 1 {
            let i2 = i1 + 1;

            let line = Line::new(self.points[i1], self.points[i2]);
            lines.push(line);
        }

        return lines;
    }

    pub(crate) fn get_bounds(&self) -> Bounds {
        return Bounds::from_points(&self.points);
    }
}
