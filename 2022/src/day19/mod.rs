use std::collections::{HashMap, hash_map::Entry};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = calc_quality_level(input);
    return result.to_string();
}

fn calc_quality_level(input: &str) -> usize {
    let blueprints = parse_input(input);
    let time = 24;
    let mut score = 0;
    for blueprint in blueprints.iter() {
        let geodes = blueprint.max_geodes(time);
        score += geodes * blueprint.index;
    }
    return score;
}

struct Blueprint {
    index: usize,
    ore_robot_ore: usize,
    clay_robot_ore: usize,
    obsidian_robot_ore: usize,
    obsidian_robot_clay: usize,
    geode_robot_ore: usize,
    geode_robot_obsidian: usize,
}

impl Blueprint {
    fn max_geodes(&self, time_limit: usize) -> usize {
        let mut state = State::new();
        state.robots.ore += 1;

        let mut cache = HashMap::new();

        let mut states = Vec::new();
        states.push((0, state));

        let mut max = 0;
        while let Some((time, state)) = states.pop() {
            if time > time_limit {
                panic!();
            }
            if time == time_limit {
                max = max.max(state.inventory.geode);
                continue;
            }

            match cache.entry(state) {
                Entry::Vacant(entry) => {
                    entry.insert(time);
                },
                Entry::Occupied(mut entry) => {
                    let cache_time = entry.get_mut();
                    if *cache_time <= time {
                        continue;
                    }
                    else {
                        *cache_time = time;
                    }
                },
            };

            {
                let mut state = state.clone();
                state.produce();
                states.push((time + 1, state));
            }

            if time < time_limit {
                if self.can_build_ore_robot(&state.inventory) {
                    let mut state = state.clone();
                    state.produce();
                    state.build_robot_ore(self);
                    states.push((time + 1, state));
                }
                if self.can_build_clay_robot(&state.inventory) {
                    let mut state = state.clone();
                    state.produce();
                    state.build_robot_clay(self);
                    states.push((time + 1, state));
                }
                if self.can_build_obsidian_robot(&state.inventory) {
                    let mut state = state.clone();
                    state.produce();
                    state.build_robot_obsidian(self);
                    states.push((time + 1, state));
                }
                if self.can_build_geode_robot(&state.inventory) {
                    let mut state = state.clone();
                    state.produce();
                    state.build_robot_geode(self);
                    states.push((time + 1, state));
                }
            }

        }

        return max;
    }

    fn can_build_ore_robot(&self, inventory: &Materials) -> bool {
        inventory.ore >= self.ore_robot_ore
    }

    fn can_build_clay_robot(&self, inventory: &Materials) -> bool {
        inventory.ore >= self.clay_robot_ore
    }

    fn can_build_obsidian_robot(&self, inventory: &Materials) -> bool {
        inventory.ore >= self.obsidian_robot_ore && inventory.clay >= self.obsidian_robot_clay
    }

    fn can_build_geode_robot(&self, inventory: &Materials) -> bool {
        inventory.ore >= self.geode_robot_ore && inventory.obsidian >= self.geode_robot_obsidian
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    robots: Materials,
    inventory: Materials,
}

impl State {
    fn build_robot_ore(&mut self, blueprint: &Blueprint) {
        self.inventory.ore -= blueprint.ore_robot_ore;
        self.robots.ore += 1;
    }

    fn build_robot_clay(&mut self, blueprint: &Blueprint) {
        self.inventory.ore -= blueprint.clay_robot_ore;
        self.robots.clay += 1;
    }
    
    fn build_robot_obsidian(&mut self, blueprint: &Blueprint) {
        self.inventory.ore -= blueprint.obsidian_robot_ore;
        self.inventory.clay -= blueprint.obsidian_robot_clay;
        self.robots.obsidian += 1;
    }

    fn build_robot_geode(&mut self, blueprint: &Blueprint) {
        self.inventory.ore -= blueprint.geode_robot_ore;
        self.inventory.obsidian -= blueprint.geode_robot_obsidian;
        self.robots.geode += 1;
    }

    fn produce(&mut self) {
        self.inventory.ore += self.robots.ore;
        self.inventory.clay += self.robots.clay;
        self.inventory.obsidian += self.robots.obsidian;
        self.inventory.geode += self.robots.geode;
    }

    fn new() -> State {
        State {
            robots: Materials::new(),
            inventory: Materials::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Materials {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Materials {
    fn new() -> Materials {
        Materials {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let regex = regex::Regex::new(
        r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$"
    ).unwrap();
    let mut blueprints = Vec::new();
    for blueprint_str in input.lines() {
        let captures = regex.captures(blueprint_str).unwrap();
        blueprints.push(Blueprint {
            index: captures[1].parse().unwrap(),
            ore_robot_ore: captures[2].parse().unwrap(),
            clay_robot_ore: captures[3].parse().unwrap(),
            obsidian_robot_ore: captures[4].parse().unwrap(),
            obsidian_robot_clay: captures[5].parse().unwrap(),
            geode_robot_ore: captures[6].parse().unwrap(),
            geode_robot_obsidian: captures[7].parse().unwrap(),
        });
    }
    return blueprints;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = calc_quality_level(input);

        assert_eq!(result, 33);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "1262");
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = calc_quality_level(input);

        assert_eq!(result, 33);
    }
}
