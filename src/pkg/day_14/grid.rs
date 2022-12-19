use std::fmt::Display;

use super::{bounds::Bounds, path::Path, xy::Xy};

#[derive(Clone)]
pub enum Contents {
    Rock,
    Sand,
    Open,
}

#[derive(Clone)]
pub struct GridPoint {
    point: Xy,
    contents: Contents,
}

impl GridPoint {
    fn new(point: Xy, state: Contents) -> Self {
        return GridPoint {
            point,
            contents: state,
        };
    }

    fn make_from_row_col_num(
        row: usize,
        col: usize,
        bounds: &Bounds,
        contents: Contents,
    ) -> Self {
        let x = col + bounds.min_x;
        let y = row + bounds.min_y; // top to bottom.

        let point = Xy::new(x, y);
        return GridPoint::new(point, contents);
    }

    fn is_solid(&self) -> bool {
        match self.contents {
            Contents::Rock => true,
            Contents::Sand => true,
            Contents::Open => false,
        }
    }

    fn set_rock(&mut self) {
        self.contents = Contents::Rock;
    }

    fn set_sand(&mut self) {
        self.contents = Contents::Sand;
    }
}

pub(crate) struct Grid {
    rows: Vec<Vec<GridPoint>>,
    bounds: Bounds,
}

impl Grid {
    fn new(points: Vec<Vec<GridPoint>>, bounds: Bounds) -> Self {
        return Grid {
            rows: points,
            bounds,
        };
    }

    pub(crate) fn make_from_rock_paths(
        paths: &Vec<Path>,
        add_floor: bool,
    ) -> Self {
        let mut all_paths: Vec<Path> = vec![];

        for p in paths {
            all_paths.push(p.clone());
        }

        let bounds = all_paths.iter().map(|p| p.get_bounds()).collect();
        let mut bounds = Bounds::union(bounds);

        if add_floor {
            let y_range = bounds.get_y_range() * 2;
            let y = bounds.max_y + 2;
            let floor = Path::new(vec![
                Xy::new(bounds.min_x - y_range, y),
                Xy::new(bounds.max_x + y_range, y),
            ]);
            all_paths.push(floor);
            let new_bounds =
                all_paths.iter().map(|p| p.get_bounds()).collect();
            bounds = Bounds::union(new_bounds);
            bounds.min_y = 0;
        }

        let mut grid = Grid::make_blank(&bounds);

        for path in all_paths {
            for point in path.get_rock_points() {
                let res = grid.set_rock(point);
                if res.is_err() {
                    panic!(); // should never happen.
                }
            }
        }

        return grid;
    }

    pub(crate) fn make_blank(bounds: &Bounds) -> Self {
        let num_cols = bounds.get_x_range() + 1; // + 1 for inclusive
        let num_rows = bounds.get_y_range() + 1; // + 1 for inclusive

        let mut rows: Vec<Vec<GridPoint>> = vec![];

        for row_num in 0..num_rows {
            let mut row: Vec<GridPoint> = vec![];

            for col_num in 0..num_cols {
                let grid_point = GridPoint::make_from_row_col_num(
                    row_num,
                    col_num,
                    bounds,
                    Contents::Open,
                );
                row.push(grid_point);
            }

            rows.push(row);
        }

        return Grid::new(rows, bounds.clone());
    }

    fn set_rock(&mut self, pt: Xy) -> Result<(), ()> {
        if !self.bounds.contains_x(pt.x) || !self.bounds.contains_y(pt.y) {
            return Err(());
        } else {
            let coords = self.get_coords(pt);

            self.rows[coords.y][coords.x].set_rock();

            return Ok(());
        }
    }

    fn set_sand(&mut self, pt: Xy) -> Result<(), ()> {
        if !self.bounds.contains_x(pt.x) || !self.bounds.contains_y(pt.y) {
            return Err(());
        } else {
            let coords = self.get_coords(pt);

            let pt = &mut self.rows[coords.y][coords.x];

            if pt.is_solid() {
                return Err(());
            } else {
                pt.set_sand();
                return Ok(());
            }
        }
    }

    fn get_point(&self, pt: Xy) -> Option<&GridPoint> {
        if !self.bounds.contains_x(pt.x) || !self.bounds.contains_y(pt.y) {
            return None;
        } else {
            let coords = self.get_coords(pt);

            return Some(&self.rows[coords.y][coords.x]);
        }
    }

    fn get_coords(&self, pt: Xy) -> Xy {
        return Xy::new(pt.x - self.bounds.min_x, pt.y - self.bounds.min_y);
    }

    pub fn get_num_sand_points_with_abyss(&mut self, x: usize) -> usize {
        let mut i: usize = 0;

        let mut res: Result<(), ()> = Ok(());

        while res.is_ok() {
            res = self.add_infinite_sand(x);
            i = i + 1;
        }

        return i - 1; // don't count the last one.
    }

    fn add_infinite_sand(&mut self, x: usize) -> Result<(), ()> {
        let sand_point = self.get_sand_point(x);
        let sand_point = self.get_point(sand_point).unwrap();

        if sand_point.is_solid() {
            self.add_row();
            return self.add_infinite_sand(x);
        } else {
            let mut pt: Xy = sand_point.point.clone();

            loop {
                let move_down = self.move_down(pt);

                if move_down.is_some() {
                    let move_down = move_down.unwrap();

                    if !move_down.is_solid() {
                        pt = move_down.point;
                        continue;
                    }
                } else {
                    return Err(());
                }

                let move_bot_left = self.move_bot_left(pt);
                if move_bot_left.is_some() {
                    let move_bot_left = move_bot_left.unwrap();

                    if !move_bot_left.is_solid() {
                        pt = move_bot_left.point;
                        continue;
                    }
                } else {
                    return Err(());
                }

                let move_bot_right = self.move_bot_right(pt);
                if move_bot_right.is_some() {
                    let move_bot_right = move_bot_right.unwrap();

                    if !move_bot_right.is_solid() {
                        pt = move_bot_right.point;
                        continue;
                    } else {
                        break;
                    }
                } else {
                    return Err(());
                }
            }

            return self.set_sand(pt);
        }
    }

    pub fn get_num_sand_points_without_abyss(&mut self, x: usize) -> usize {
        let mut i: usize = 0;

        let mut res: Result<(), ()> = Ok(());

        while res.is_ok() {
            res = self.add_finite_sand(x);
            i = i + 1;
        }

        return i - 1; // don't count the last one.
    }

    fn add_finite_sand(&mut self, x: usize) -> Result<(), ()> {
        let sand_point = Xy::new(x, 0);
        let sand_point = self.get_point(sand_point).unwrap();

        if sand_point.is_solid() {
            return Err(());
        } else {
            let mut pt: Xy = sand_point.point.clone();

            loop {
                let move_down = self.move_down(pt);

                if move_down.is_some() {
                    let move_down = move_down.unwrap();

                    if !move_down.is_solid() {
                        pt = move_down.point;
                        continue;
                    }
                }

                let move_bot_left = self.move_bot_left(pt);
                if move_bot_left.is_some() {
                    let move_bot_left = move_bot_left.unwrap();

                    if !move_bot_left.is_solid() {
                        pt = move_bot_left.point;
                        continue;
                    }
                }

                let move_bot_right = self.move_bot_right(pt);
                if move_bot_right.is_some() {
                    let move_bot_right = move_bot_right.unwrap();

                    if !move_bot_right.is_solid() {
                        pt = move_bot_right.point;
                        continue;
                    }
                }

                break;
            }

            return self.set_sand(pt);
        }
    }

    fn get_sand_point(&self, x: usize) -> Xy {
        return Xy::new(x, self.bounds.min_y);
    }

    // add a row to the list of points at the top.
    fn add_row(&mut self) {
        let mut new_rows: Vec<Vec<GridPoint>> = vec![self.make_top_row()];

        for old_row in &self.rows {
            new_rows.push(old_row.clone());
        }

        self.bounds.min_y = self.bounds.min_y - 1;
        self.rows = new_rows;
    }

    fn make_top_row(&self) -> Vec<GridPoint> {
        let num_cols = self.bounds.get_x_range() + 1;

        let mut new_row: Vec<GridPoint> = vec![];

        for col in 0..num_cols {
            new_row.push(GridPoint::new(
                Xy::new(col + self.bounds.min_x, self.bounds.min_y - 1),
                Contents::Open,
            ));
        }

        return new_row;
    }

    fn move_down(&self, xy: Xy) -> Option<&GridPoint> {
        let down = Xy::new(xy.x, xy.y + 1);
        return self.get_point(down);
    }

    fn move_bot_left(&self, xy: Xy) -> Option<&GridPoint> {
        let bot_left = Xy::new(xy.x - 1, xy.y + 1);
        return self.get_point(bot_left);
    }

    fn move_bot_right(&self, xy: Xy) -> Option<&GridPoint> {
        let bot_right = Xy::new(xy.x + 1, xy.y + 1);
        return self.get_point(bot_right);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dot = char::from_u32(183).unwrap();
        for row in &self.rows {
            let mut str: Vec<char> = vec![];

            for col in row {
                let c = match col.contents {
                    Contents::Rock => '#',
                    Contents::Sand => 'O',
                    Contents::Open => dot,
                };

                str.push(c);
            }

            let result = writeln!(f, "{}", str.iter().collect::<String>());
            if result.is_err() {
                return result;
            }
        }

        return Ok(());
    }
}
