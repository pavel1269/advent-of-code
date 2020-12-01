
mod d01;

fn main() {
    let result = d01::get_solution_day01_part1();

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_part1() {
        let expected_result = 545379;
        let result = d01::get_solution_day01_part1();

        assert_eq!(expected_result, result);
    }
}
