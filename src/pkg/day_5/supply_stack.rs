use std::collections::VecDeque;

#[derive(Clone)]
pub(crate) struct SupplyStack {
    crates: VecDeque<char>,
}

impl SupplyStack {
    pub(crate) fn new() -> Self {
        return SupplyStack {
            crates: VecDeque::new(),
        };
    }

    pub(crate) fn push(&mut self, item: char) {
        self.crates.push_front(item);
    }

    pub(crate) fn pop(&mut self) -> char {
        return self.crates.pop_front().unwrap();
    }

    pub(crate) fn peek_top(&self) -> char {
        return *self.crates.front().unwrap();
    }
}
