
use super::instruction_runner::State;

pub trait Instruction {
    fn apply(&self, state: &mut State);
}

pub struct MemoryInstruction {
    pub address: u64,
    pub value: u64,
}

impl Instruction for MemoryInstruction {
    fn apply(&self, state: &mut State) {
        let new_value = self.value & state.mask_and | state.mask_or;
        // println!("Writing [{}] = {} ({:#b}, {:#b})", self.address, new_value, state.mask_and, state.mask_or);
        state.memory.insert(self.address, new_value);
    }
}

pub struct MaskInstruction {
    pub mask_or: u64,
    pub mask_and: u64,
}

impl Default for MaskInstruction {
    fn default() -> Self {
        Self {
            mask_or: 0b0,
            mask_and: !0b0,
        }
    }
}

impl Instruction for MaskInstruction {
    fn apply(&self, state: &mut State) {
        state.mask_and = self.mask_and;
        state.mask_or = self.mask_or;
    }
}
