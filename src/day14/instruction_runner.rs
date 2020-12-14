
use super::*;
use std::collections::HashMap;

pub struct State {
    pub mask_and: u64,
    pub mask_or: u64,
    pub memory: HashMap<u64, u64>,
}

pub fn run_program(input: &str) -> State {
    let instructions = parse_input_program(input);
    let mut state = State {
        mask_and: !0,
        mask_or: 0,
        memory: HashMap::new(),
    };

    for instuction in instructions.iter() {
        instuction.apply(&mut state);
    }

    return state;
}
