
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
    fn example_run_program_v1_memory_sum() {
        let input = get_example_input();
        let state = run_program(input, Version::V1);
        let result = sum_memory(&state);

        assert_eq!(165, result);
    }

    fn get_example2_input() -> &'static str {
"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
    }

    #[test]
    fn example2_run_program_v2_memory_sum() {
        let input = get_example2_input();
        let state = run_program(input, Version::V2);
        let result = sum_memory(&state);

        assert_eq!(208, result);
    }
}
