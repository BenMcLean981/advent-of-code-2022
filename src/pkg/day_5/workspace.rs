use super::{move_order::MoveOrder, supply_stack::SupplyStack};

pub(crate) struct Workspace {
    stacks: Vec<SupplyStack>,
}

impl Workspace {
    pub(crate) fn make_of_size(size: usize) -> Workspace {
        let stacks: Vec<SupplyStack> = vec![SupplyStack::new(); size];

        return Workspace { stacks };
    }

    pub(crate) fn push_to(&mut self, idx: usize, item: char) {
        self.stacks[idx].push(item);
    }

    pub(crate) fn apply_crate_mover_9000_order(&mut self, order: MoveOrder) {
        for _ in 0..order.quantity {
            self.move_single_item(&order)
        }
    }

    fn move_single_item(&mut self, order: &MoveOrder) {
        let item = self.stacks[order.from].pop();
        self.stacks[order.to].push(item);
    }

    pub(crate) fn apply_crate_mover_9001_order(&mut self, order: MoveOrder) {
        let mut substack: Vec<char> = vec![];

        for _ in 0..order.quantity {
            substack.push(self.stacks[order.from].pop());
        }

        substack.reverse();

        for c in substack {
            self.stacks[order.to].push(c);
        }
    }

    pub(crate) fn get_tops(&self) -> String {
        return self.stacks.iter().map(|s| s.peek_top()).collect();
    }
}
