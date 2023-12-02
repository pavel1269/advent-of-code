use std::collections::HashMap;

fn main() {
    let input = get_input();
    let games = parse_input(input);
    let result_part1 = sum_matching_games(&games);
    println!("Result part 1: {}", result_part1);

    let result_part2 = sum_game_power(&games);
    println!("Result part 2: {}", result_part2);
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Cubes,
}

#[derive(Debug)]
struct Cubes {
    cubes: HashMap<String, u32>,
}

impl Cubes {
    fn add_known(&mut self, color: String, count: u32) {
        self.cubes.entry(color).and_modify(|recorded_count| {
            *recorded_count = count.max(*recorded_count);
        }).or_insert(count);
    }

    fn fits(&self, allowance: &Cubes) -> bool {
        for (color, allowance_amount) in allowance.cubes.iter() {
            if !self.cubes.contains_key(color) {
                continue;
            }

            let amount = self.cubes[color];
            if amount > *allowance_amount {
                return false;
            }
        }
        return true;
    }

    fn power_sum(&self) -> u64 {
        let mut sum = 1;
        for (_, amount) in self.cubes.iter() {
            let amount_u64: u64 = (*amount).into();
            sum *= amount_u64;
        }
        return sum;
    }
}

fn sum_game_power(games: &Vec<Game>) -> u64 {
    let mut sum = 0;
    for game in games {
        sum += game.cubes.power_sum();
    }

    return sum;
}

fn sum_matching_games(games: &Vec<Game>) -> u32 {
    let mut sum = 0;

    let allowance = get_matching();
    for game in games {
        if game.cubes.fits(&allowance) {
            sum += game.id;
        }
    }

    return sum;
}

fn get_matching() -> Cubes {
    return Cubes {
        cubes: HashMap::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ]),
    };
}

fn parse_input(input: &str) -> Vec<Game> {
    let regex_game = regex::Regex::new(r"^Game (\d+):(.+)$").unwrap();
    let regex_color = regex::Regex::new(r"^(\d+) (.+)$").unwrap();
    let input_lines = input.lines();
    let mut games = Vec::new();
    input_lines.for_each(|line| {
        let game_captures = regex_game.captures(line).unwrap();
        let game_id = game_captures[1].parse().unwrap();
        let game_subsets = game_captures[2].split(';');
        let mut game_cubes = Cubes {
            cubes: HashMap::new(),
        };
        game_subsets.for_each(|subset| {
            subset.split(',').for_each(|cubes| {
                let cubes_captures = regex_color.captures(cubes.trim()).unwrap();
                let count: u32 = cubes_captures[1].parse().unwrap();
                let color = cubes_captures[2].to_string();
                game_cubes.add_known(color, count);
            });
        });

        let game = Game {
            id: game_id,
            cubes: game_cubes,
        };
        games.push(game);
    });

    return games;
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
        let games = parse_input(input);
        let result = sum_matching_games(&games);

        assert_eq!(result, 8);
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let games = parse_input(input);
        let result = sum_game_power(&games);

        assert_eq!(result, 2286);
    }
}
