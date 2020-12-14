
pub mod instructions;
#[path = "instruction_parser.rs"] pub mod instruction_parser;
#[path = "instruction_runner.rs"] pub mod instruction_runner;

pub use instructions::*;
pub use instruction_parser::*;
pub use instruction_runner::*;

pub fn sum_memory(state: &State) -> u64 {
    state.memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
    }

    #[test]
    fn example_get_earliest_bus() {
        let input = get_example_input();
        let state = run_program(input);
        let result = sum_memory(&state);

        assert_eq!(165, result);
    }
}
