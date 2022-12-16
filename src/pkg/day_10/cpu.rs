#[derive(Clone)]
pub(crate) struct Cpu {
    pub x: isize,
}

impl Cpu {
    pub(crate) fn new(x: isize) -> Self {
        return Cpu { x };
    }

    pub(crate) fn initialize() -> Self {
        return Cpu::new(1);
    }
}
