use std::cmp;

use super::tree::Tree;

pub(crate) struct Grid {
    rows: Vec<Vec<Tree>>,
}

impl Grid {
    pub(crate) fn new(rows: Vec<Vec<Tree>>) -> Self {
        return Grid { rows };
    }

    fn get_col(&self, col: usize) -> Vec<Tree> {
        return self.rows.iter().map(|r| r[col]).collect();
    }

    fn get_row(&self, row: usize) -> Vec<Tree> {
        return self.rows[row].clone();
    }

    fn get_trees(&self) -> Vec<&Tree> {
        return self.rows.iter().flat_map(|r| r).collect();
    }

    pub(crate) fn get_all_visible(&self) -> Vec<&Tree> {
        return self
            .get_trees()
            .iter()
            .filter(|t| self.is_visible(t))
            .map(|t| *t)
            .collect();
    }

    fn is_visible(&self, tree: &Tree) -> bool {
        return Grid::are_all_shorter(self.get_trees_above(tree), tree)
            || Grid::are_all_shorter(self.get_trees_below(tree), tree)
            || Grid::are_all_shorter(self.get_trees_left(tree), tree)
            || Grid::are_all_shorter(self.get_trees_right(tree), tree);
    }

    fn are_all_shorter(trees: Vec<Tree>, tree: &Tree) -> bool {
        return trees.iter().all(|t| t.size < tree.size);
    }

    pub(crate) fn get_best_scenic_score(&self) -> usize {
        return self.get_trees().iter().fold(0, |score, tree| {
            cmp::max(score, self.get_scenic_score(tree))
        });
    }

    fn get_scenic_score(&self, tree: &Tree) -> usize {
        let above_score = Grid::get_score(tree, self.get_trees_above(tree));
        let below_score = Grid::get_score(tree, self.get_trees_below(tree));
        let left_score = Grid::get_score(tree, self.get_trees_left(tree));
        let right_score = Grid::get_score(tree, self.get_trees_right(tree));

        return above_score * below_score * left_score * right_score;
    }

    fn get_score(tree: &Tree, line: Vec<Tree>) -> usize {
        let first_taller = line.iter().position(|t| t.size >= tree.size);

        match first_taller {
            Some(pos) => pos + 1,
            None => line.len(),
        }
    }

    fn get_trees_above(&self, tree: &Tree) -> Vec<Tree> {
        let col = self.get_col(tree.col);

        let mut above = col[0..tree.row].to_vec();
        above.reverse();

        return above;
    }

    fn get_trees_below(&self, tree: &Tree) -> Vec<Tree> {
        let col = self.get_col(tree.col);

        return col[tree.row + 1..].to_vec();
    }

    fn get_trees_left(&self, tree: &Tree) -> Vec<Tree> {
        let row = self.get_row(tree.row);

        let mut left = row[0..tree.col].to_vec();
        left.reverse();

        return left;
    }

    fn get_trees_right(&self, tree: &Tree) -> Vec<Tree> {
        let row = self.get_row(tree.row);

        return row[tree.col + 1..].to_vec();
    }
}
