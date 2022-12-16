use super::super::cpu::Cpu;
use super::instruction::Instruction;

pub(crate) struct Noop {}

impl Noop {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Instruction for Noop {
    fn simulate(&self, cpu: &Cpu) -> Vec<Cpu> {
        return vec![cpu.clone()];
    }
}
