use super::tree::Tree;

pub(crate) struct Grid {
    rows: Vec<Vec<Tree>>,
    num_rows: usize,
    num_cols: usize,
}

impl Grid {
    pub(crate) fn new(
        rows: Vec<Vec<Tree>>,
        num_rows: usize,
        num_cols: usize,
    ) -> Self {
        return Grid {
            rows,
            num_rows,
            num_cols,
        };
    }

    pub(crate) fn get_all_visible(&self) -> Vec<&Tree> {
        return self
            .get_trees()
            .iter()
            .filter(|t| self.is_visible(t))
            .map(|t| *t)
            .collect();
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

    fn is_hidden(&self, tree: &Tree) -> bool {
        return !self.is_visible(tree);
    }

    fn is_visible(&self, tree: &Tree) -> bool {
        let is_visible_in_row = self.is_visible_in_row(tree);
        let is_visible_in_col = self.is_visible_in_col(tree);

        return is_visible_in_row || is_visible_in_col;
    }

    fn is_visible_in_row(&self, tree: &Tree) -> bool {
        let row = self.get_row(tree.row);

        let before = row[0..tree.col].to_vec();
        let after = row[tree.col + 1..].to_vec();

        return Grid::are_all_shorter(before, tree.size)
            || Grid::are_all_shorter(after, tree.size);
    }

    fn is_visible_in_col(&self, tree: &Tree) -> bool {
        let col = self.get_col(tree.col);

        let before = col[0..tree.row].to_vec();
        let after = col[tree.row + 1..].to_vec();

        return Grid::are_all_shorter(before, tree.size)
            || Grid::are_all_shorter(after, tree.size);
    }

    fn are_all_shorter(trees: Vec<Tree>, size: usize) -> bool {
        return trees.iter().all(|t| t.size < size);
    }
}
