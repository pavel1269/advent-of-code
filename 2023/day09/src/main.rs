fn main() {
    let input = get_input();
    let sequences = parse_input(input);

    let result_part1 = process_sequences(&sequences);
    println!("Part1: {}", result_part1);
}

fn process_sequences(sequences: &Vec<Vec<i32>>) -> i32 {
    sequences.iter().map(|sequence| process_sequnce(sequence)).sum()
}

fn process_sequnce(sequence: &Vec<i32>) -> i32 {
    let mut pyramid = Vec::new();
    pyramid.push(sequence.clone());

    while !is_sequence_stable(pyramid.last().unwrap()) {
        let last_sequence = pyramid.last().unwrap();
        let mut new_row = Vec::with_capacity(last_sequence.len());

        for index in 0..last_sequence.len() - 1 {
            let a = last_sequence[index];
            let b = last_sequence[index + 1];
            new_row.push(b - a);
        }
        pyramid.push(new_row);
    }

    let mut difference = *pyramid.last().unwrap().last().unwrap();
    for index in (0..pyramid.len() - 1).rev() {
        let new = pyramid[index].last().unwrap() + difference;
        difference = new;
        pyramid[index].push(new);
    }

    return difference;
}

fn is_sequence_stable(sequence: &Vec<i32>) -> bool {
    let first = *sequence.first().unwrap();
    return sequence.iter().all(|a| *a == first);
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| line.split(' ').map(|str| str.parse().unwrap()).collect()).collect()
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let sequences = parse_input(input);
        let result = process_sequences(&sequences);
        assert_eq!(result, 114);
    }
}
