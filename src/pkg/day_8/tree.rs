#[derive(Clone, Copy)]
pub(crate) struct Tree {
    pub size: usize,
    pub col: usize,
    pub row: usize,
}

impl Tree {
    pub(crate) fn new(size: usize, x_pos: usize, y_pos: usize) -> Tree {
        return Tree {
            size,
            col: x_pos,
            row: y_pos,
        };
    }
}
