pub(crate) struct MoveOrder {
    pub quantity: usize,
    pub from: usize,
    pub to: usize,
}

impl MoveOrder {
    pub(crate) fn new(quantity: usize, from: usize, to: usize) -> Self {
        return MoveOrder {
            quantity,
            from: from - 1,
            to: to - 1,
        };
    }
}
