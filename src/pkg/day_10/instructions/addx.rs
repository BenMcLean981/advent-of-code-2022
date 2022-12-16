use super::super::cpu::Cpu;
use super::instruction::Instruction;

pub(crate) struct Addx {
    x: isize,
}

impl Addx {
    pub(crate) fn new(x: isize) -> Self {
        Self { x }
    }
}

impl Instruction for Addx {
    fn simulate(&self, cpu: &Cpu) -> Vec<Cpu> {
        return vec![cpu.clone(), Cpu::new(cpu.x + self.x)];
    }
}
