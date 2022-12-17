use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use super::{
    node::{Height, Node},
    path::Path,
};

pub(crate) struct Graph {
    rows: Vec<Vec<Rc<RefCell<Node>>>>,
}

impl Graph {
    pub fn new(rows: Vec<Vec<Node>>) -> Self {
        let mut row_rcs: Vec<Vec<Rc<RefCell<Node>>>> = vec![];

        for r in rows {
            let mut row_rc: Vec<Rc<RefCell<Node>>> = vec![];

            for n in r {
                row_rc.push(Rc::new(RefCell::new(n)));
            }

            row_rcs.push(row_rc);
        }

        return Graph { rows: row_rcs };
    }

    fn get_nodes(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut nodes: Vec<Rc<RefCell<Node>>> = vec![];

        for r in &self.rows {
            for n in r {
                nodes.push(Rc::clone(n));
            }
        }

        return nodes;
    }

    pub fn find_shortest_path(
        &self,
        start: Rc<RefCell<Node>>,
    ) -> Option<Path> {
        let mut explored: HashSet<Node> = HashSet::new();
        explored.insert(start.as_ref().borrow().clone());

        let base_path = Path::initialize(start);

        let mut paths: VecDeque<Path> = VecDeque::new();
        paths.push_back(base_path);

        while !paths.is_empty() {
            let to_grow = paths.pop_back().unwrap();

            if to_grow.is_done() {
                return Some(to_grow);
            } else {
                let tail = to_grow.get_tail();
                let neighbors =
                    self.get_accessible_neighbors(&tail, &explored);

                for n in neighbors {
                    explored.insert(n.as_ref().borrow().clone());
                    paths.push_front(to_grow.add(&n))
                }
            }
        }

        return None;
    }

    pub fn get_start(&self) -> Rc<RefCell<Node>> {
        let nodes = self.get_nodes();

        let start = nodes
            .iter()
            .find(|n| n.as_ref().borrow().height == Height::Start)
            .unwrap();

        return Rc::clone(start);
    }

    pub fn get_zero_heights(&self) -> Vec<Rc<RefCell<Node>>> {
        return self
            .get_nodes()
            .iter()
            .filter(|n| n.as_ref().borrow().height == Height::NodeHeight(0))
            .map(|n| Rc::clone(n))
            .collect();
    }

    fn get_accessible_neighbors(
        &self,
        node: &Rc<RefCell<Node>>,
        explored: &HashSet<Node>,
    ) -> Vec<Rc<RefCell<Node>>> {
        let neighbors = self.get_neighbors(node);

        return neighbors
            .iter()
            .filter(|n| {
                node.as_ref().borrow().can_reach(n.as_ref().borrow().height)
            })
            .filter(|n| !explored.contains(&n.as_ref().borrow()))
            .map(|n| Rc::clone(n))
            .collect();
    }

    fn get_neighbors(
        &self,
        node: &Rc<RefCell<Node>>,
    ) -> Vec<Rc<RefCell<Node>>> {
        let row: isize = isize::try_from(node.as_ref().borrow().row).unwrap();
        let col: isize = isize::try_from(node.as_ref().borrow().col).unwrap();

        let pot_neighbors = vec![
            self.get_node(row, col - 1),
            self.get_node(row - 1, col),
            self.get_node(row, col + 1),
            self.get_node(row + 1, col),
        ];

        return pot_neighbors
            .iter()
            .filter(|x| x.is_some())
            .map(|x| Rc::clone(x.as_ref().unwrap()))
            .collect();
    }

    fn get_node(&self, row: isize, col: isize) -> Option<Rc<RefCell<Node>>> {
        if !self.has_col(col) || !self.has_row(row) {
            return None;
        } else {
            let row = usize::try_from(row).unwrap();
            let col = usize::try_from(col).unwrap();

            let node = Rc::clone(&self.rows[row][col]);

            return Some(node);
        }
    }

    fn has_row(&self, row: isize) -> bool {
        let num_rows = isize::try_from(self.rows.len()).unwrap();
        return row >= 0 && row <= num_rows - 1;
    }

    fn has_col(&self, col: isize) -> bool {
        let num_cols = isize::try_from(self.rows[0].len()).unwrap();
        return col >= 0 && col <= num_cols - 1;
    }
}
