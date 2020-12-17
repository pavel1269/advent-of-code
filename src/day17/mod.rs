
pub mod coordinates;
pub mod space;
use space::*;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let mut space = Space::from_input(input);

    // space.print();
    space.simlate_cycle();
    space.simlate_cycle();
    space.simlate_cycle();
    space.simlate_cycle();
    space.simlate_cycle();
    space.simlate_cycle();
    // space.print();

    let result = space.count_actives();
    return result as i64;
}

fn get_challenge_input() -> &'static str {
    ".###.#.#
####.#.#
#.....#.
####....
#...##.#
########
..#####.
######.#
"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_get_part1_result() {
        let result = get_part1_result();

        assert_eq!(388, result);
    }
}
