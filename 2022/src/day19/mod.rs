
pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = calc_quality_level(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = calc_quality_level_top3(input);
    return result.to_string();
}

fn calc_quality_level_top3(input: &str) -> usize {
    let blueprints = parse_input(input);
    let time = 32;
    let mut score = 1;
    for blueprint in blueprints.iter().take(3) {
        let geodes = blueprint.max_geodes(time);
        score *= geodes;
    }
    return score;
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

        let mut states = Vec::new();
        states.push((0, state));

        let max_ore_needed = self
            .ore_robot_ore
            .max(self.clay_robot_ore)
            .max(self.obsidian_robot_ore)
            .max(self.geode_robot_ore);
        let mut max = 0;
        while let Some((time, state)) = states.pop() {
            if time > time_limit {
                panic!();
            }

            if time == time_limit {
                max = max.max(state.inventory.geode);
                continue;
            }
            if time + 1 == time_limit {
                let mut state = state.clone();
                state.produce();
                max = max.max(state.inventory.geode);
                continue;
            }

            let mut include_wait = false;
            if max_ore_needed > state.robots.ore {
                let mut state = state.clone();
                let mut time = time;
                let mut can_build_ore = false;
                while !can_build_ore && time < time_limit {
                    can_build_ore = self.can_build_ore_robot(&state.inventory);
                    time += 1;
                    state.produce();
                }
                if can_build_ore {
                    state.build_robot_ore(self);
                    states.push((time, state));
                }
                else {
                    include_wait = true;
                }
            }
            if self.obsidian_robot_clay > state.robots.clay {
                let mut state = state.clone();
                let mut time = time;
                let mut can_build_clay = false;
                while !can_build_clay && time < time_limit {
                    can_build_clay = self.can_build_clay_robot(&state.inventory);
                    time += 1;
                    state.produce();
                }
                if can_build_clay {
                    state.build_robot_clay(self);
                    states.push((time, state));
                }
                else {
                    include_wait = true;
                }
            }
            if self.geode_robot_obsidian > state.robots.obsidian && self.can_build_obsidian_robot_ever(&state.robots) {
                let mut state = state.clone();
                let mut time = time;
                let mut can_build_obsidian = false;
                while !can_build_obsidian && time < time_limit {
                    can_build_obsidian = self.can_build_obsidian_robot(&state.inventory);
                    time += 1;
                    state.produce();
                }
                if can_build_obsidian {
                    state.build_robot_obsidian(self);
                    states.push((time, state));
                }
                else {
                    include_wait = true;
                }
            }
            if self.can_build_geode_robot_ever(&state.robots) {
                let mut state = state.clone();
                let mut time = time;
                let mut can_build_geode = false;
                while !can_build_geode && time < time_limit {
                    can_build_geode = self.can_build_geode_robot(&state.inventory);
                    time += 1;
                    state.produce();
                }
                if can_build_geode {
                    state.build_robot_geode(self);
                    states.push((time, state));
                }
                else {
                    include_wait = true;
                }
            }
            if include_wait {
                let mut state = state.clone();
                let mut time = time;
                while time < time_limit {
                    time += 1;
                    state.produce();
                }
                states.push((time, state));
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

    fn can_build_obsidian_robot_ever(&self, robots: &Materials) -> bool {
        robots.ore > 0 && robots.clay > 0
    }

    fn can_build_geode_robot(&self, inventory: &Materials) -> bool {
        inventory.ore >= self.geode_robot_ore && inventory.obsidian >= self.geode_robot_obsidian
    }

    fn can_build_geode_robot_ever(&self, robots: &Materials) -> bool {
        robots.ore > 0 && robots.obsidian > 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
        let result = calc_quality_level_top3(input);

        assert_eq!(result, 56 * 62);
    }
    
    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "37191");
    }
}
