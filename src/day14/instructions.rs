
use super::{Version, instruction_runner::State};

pub trait Instruction {
    fn apply(&self, state: &mut State);
}

pub struct MemoryInstruction {
    pub address: u64,
    pub value: u64,
}

impl Instruction for MemoryInstruction {
    fn apply(&self, state: &mut State) {
        if state.version == Version::V1 {
            let new_value = self.value & state.mask_and | state.mask_or;
            // println!("Writing [{}] = {} ({:#b}, {:#b})", self.address, new_value, state.mask_and, state.mask_or);
            state.memory.insert(self.address, new_value);
        } else {
            let floating_mask = state.mask_and ^ state.mask_or;
            let mut floating: Vec<usize> = Vec::new();
            for index in 0..36 {
                let mask = 1 << index;
                if floating_mask & mask != 0 {
                    floating.push(index);
                }
            }
            //println!("{:#b} ^ {:#b} = {:#b} -> {:?}", state.mask_and, state.mask_or, floating_mask, floating);
            for variable_number in 0..u64::pow(2, floating.len() as u32) {
                let mut address = self.address | state.mask_or;
                // println!("Saving (1) [{} {:#016b}] = {} ({:#016b})", address, address, self.value, variable_number);
                for (index, index_float) in floating.iter().enumerate() {
                    let mask = 1 << index;
                    if variable_number & mask != 0 {
                        address |= 1 << index_float;
                        // println!("Setting '1' to index {} (mask: {:#08b})", index_float, mask);
                    } else {
                        address &= !(1 << index_float);
                        // println!("Setting '0' to index {} (mask: {:#08b})", index_float, mask);
                    }
                }
                // println!("Saving (2) [{} {:#016b}] = {}", address, address, self.value);
                state.memory.insert(address, self.value);
            }
        }
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
