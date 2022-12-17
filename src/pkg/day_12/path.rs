use std::{cell::RefCell, rc::Rc};

use super::node::{Height, Node};

#[derive(Clone)]
pub(crate) struct Path {
    nodes: Vec<Rc<RefCell<Node>>>,
}

impl Path {
    pub fn new(nodes: Vec<Rc<RefCell<Node>>>) -> Self {
        return Path { nodes };
    }

    pub fn initialize(node: Rc<RefCell<Node>>) -> Self {
        return Path::new(vec![node]);
    }

    pub fn is_done(&self, height: Height) -> bool {
        return self.get_tail().as_ref().borrow().height == height;
    }

    pub fn get_tail(&self) -> Rc<RefCell<Node>> {
        return Rc::clone(self.nodes.last().unwrap());
    }

    pub fn add(&self, node: &Rc<RefCell<Node>>) -> Path {
        let mut nodes: Vec<Rc<RefCell<Node>>> =
            self.nodes.iter().map(|n| Rc::clone(n)).collect();

        nodes.push(Rc::clone(node));

        return Path::new(nodes);
    }

    pub fn len(&self) -> usize {
        return self.nodes.len() - 1; // exclude end.
    }
}
