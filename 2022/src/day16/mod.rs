use std::collections::{HashMap, hash_map::Entry, HashSet};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = get_max_release(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = get_max_release_with_elephant(input);
    return result.to_string();
}

fn get_max_release_with_elephant(input: &str) -> usize {
    let limit = 26;
    let pipes = parse_input(input);
    let distances = build_connection_map(&pipes);

    let mut targets = HashSet::new();
    for pipe in pipes.iter() {
        if pipe.flow > 0 {
            targets.insert(pipe.name.clone());
        }
    }

    let mut options = Vec::new();
    let mut best_flow = 0;
    options.push(("AA".to_string(), 0, "AA".to_string(), 0, 0, 0, 0, targets));
    while let Some((place, place_distance, place_elephant, place_elephant_distance, minutes, flow, flow_min, targets)) = options.pop() {
        let walking_distance = place_distance.min(place_elephant_distance);
        if minutes + walking_distance >= limit {
            let flow = flow + flow_min * (limit - minutes);
            best_flow = best_flow.max(flow);
            continue;
        }
        
        let i_reached = place_distance == walking_distance;
        let elephant_reached = place_elephant_distance == walking_distance;

        let mut flow_new = flow + flow_min * walking_distance;
        let mut flow_min_new = flow_min;
        let mut minutes_new = minutes + walking_distance;

        if i_reached {
            let pipe = pipes.iter().filter(|p| p.name == place).nth(0).unwrap();
            flow_min_new += pipe.flow;
        }
        if elephant_reached {
            let pipe = pipes.iter().filter(|p| p.name == place_elephant).nth(0).unwrap();
            flow_min_new += pipe.flow;
        }

        if targets.len() == 0 {
            if !i_reached && minutes + place_distance < limit {
                let walking_distance = place_distance - (minutes_new - minutes);
                let pipe = pipes.iter().filter(|p| p.name == place).nth(0).unwrap();
                flow_new = flow_new + flow_min_new * walking_distance;
                flow_min_new += pipe.flow;
                minutes_new += walking_distance;
            }
            else if !elephant_reached && minutes + place_elephant_distance < limit {
                let walking_distance = place_elephant_distance - (minutes_new - minutes);
                let pipe = pipes.iter().filter(|p| p.name == place_elephant).nth(0).unwrap();
                flow_new = flow_new + flow_min_new * walking_distance;
                flow_min_new += pipe.flow;
                minutes_new += walking_distance;
            }

            let final_flow = flow_new + flow_min_new * (limit - minutes_new);
            best_flow = best_flow.max(final_flow);
            continue;
        }

        if i_reached || elephant_reached {
            for target in targets.iter().cloned() {
                let distance_i = *distances.get(&(place.clone(), target.clone())).unwrap() + 1;
                let distance_elephant = *distances.get(&(place_elephant.clone(), target.clone())).unwrap() + 1;
                let mut targets_new = targets.clone();
                targets_new.remove(&target);

                if i_reached && elephant_reached {
                    // Both of us reached a point
                    
                    if targets.len() > 1 {
                        for target2 in targets.iter().cloned() {
                            if &target == &target2 {
                                continue;
                            }

                            let distance_elephant = *distances.get(&(place_elephant.clone(), target2.clone())).unwrap() + 1;
                            let mut targets_new = targets_new.clone();
                            targets_new.remove(&target2);
                            options.push((target.clone(), distance_i, target2, distance_elephant, minutes_new, flow_new, flow_min_new, targets_new));
                        }
                    }
                    else {
                        // Cannot pick a second target so either of us walks there
                        options.push((target.clone(), distance_i, place_elephant.clone(), limit + 1, minutes_new, flow_new, flow_min_new, targets_new.clone()));
                        options.push((place.clone(), limit + 1, target.clone(), distance_elephant, minutes_new, flow_new, flow_min_new, targets_new));
                    }
                }
                else {
                    if i_reached {
                        // Only I reached a point
                        options.push((target.clone(), distance_i, place_elephant.clone(), place_elephant_distance - walking_distance, minutes_new, flow_new, flow_min_new, targets_new));
                    }
                    else {
                        // Only elephant reached a point
                        options.push((place.clone(), place_distance - walking_distance, target.clone(), distance_elephant, minutes_new, flow_new, flow_min_new, targets_new));
                    }
                }
            }
        }
    }
    return best_flow;
}

fn get_max_release(input: &str) -> usize {
    let limit = 30;
    let pipes = parse_input(input);
    let distances = build_connection_map(&pipes);

    let mut targets = HashSet::new();
    for pipe in pipes.iter() {
        if pipe.flow > 0 {
            targets.insert(pipe.name.clone());
        }
    }

    let mut options = Vec::new();
    let mut best_flow = 0;
    options.push(("AA".to_string(), 0, 0, 0, targets));
    while let Some((place, minutes, flow, flow_min, targets)) = options.pop() {
        if targets.len() > 0 {
            for target in targets.iter().cloned() {
                let distance = distances.get(&(place.clone(), target.clone())).unwrap();
                let time_open = minutes + distance + 1;
                if time_open >= limit {
                    let flow = flow + flow_min * (limit - minutes);
                    best_flow = best_flow.max(flow);
                }
                else {
                    let pipe = pipes.iter().filter(|p| p.name == target).nth(0).unwrap();
                    let mut targets_new = targets.clone();
                    targets_new.remove(&target);
                    options.push((target, time_open, flow + flow_min * (distance + 1), flow_min + pipe.flow, targets_new));
                }
            }
        }
        else {
            let flow = flow + flow_min * (limit - minutes);
            best_flow = best_flow.max(flow);
        }
    }
    return best_flow;
}

fn build_connection_map(pipes: &Vec<Pipe>) -> HashMap<(String, String), usize> {
    let mut distances = HashMap::new();
    for pipe in pipes.iter() {
        distances.insert((pipe.name.clone(), pipe.name.clone()), 0);
        for connection in pipe.connections.iter().cloned() {
            distances.insert((pipe.name.clone(), connection), 1);
        }
    }
    let mut change = true;
    while change {
        change = false;
        for pipe1 in pipes.iter().map(|p| p.name.clone()) {
            for pipe2 in pipes.iter().map(|p| p.name.clone()) {
                if &pipe1 == &pipe2 {
                    continue;
                }
                let key_12 = (pipe1.clone(), pipe2.clone());
                let known_12 = distances.contains_key(&key_12);
                for pipe3 in pipes.iter().map(|p| p.name.clone()) {
                    if &pipe1 == &pipe3 || &pipe2 == &pipe3 {
                        continue;
                    }

                    let key_13 = (pipe1.clone(), pipe3.clone());
                    let key_23 = (pipe2.clone(), pipe3.clone());
                    let known_23 = distances.contains_key(&key_23);

                    if known_12 && known_23 {
                        let distance_12 = *distances.get(&key_12).unwrap();
                        let distance_23 = *distances.get(&key_23).unwrap();
                        let distance_13 = distance_12 + distance_23;
                        match distances.entry(key_13) {
                            Entry::Vacant(entry) => {
                                entry.insert(distance_12 + distance_23);
                                change = true;
                            },
                            Entry::Occupied(mut entry) => {
                                if entry.get() > &distance_13 {
                                    *entry.get_mut() = distance_13;
                                    change = true;
                                }
                            },
                        }
                    }
                }
            }
        }
    }
    return distances
}

struct Pipe {
    name: String,
    flow: usize,
    connections: Vec<String>,
}

fn parse_input(input: &str) -> Vec<Pipe> {
    let regex = regex::Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
    let mut pipes = Vec::new();
    for line in input.lines() {
        let captures = match regex.captures(line) {
            Some(captures) => captures,
            None => panic!("Failed to parse: '{}'", line),
        };
        let name = captures[1].to_string();
        let flow = captures[2].parse().unwrap();
        let connections = captures[3].split(", ").map(|pipe| pipe.to_string()).collect();

        let pipe = Pipe {
            name,
            flow,
            connections,
        };
        pipes.push(pipe);
    }
    return pipes;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = get_max_release(input);

        assert_eq!(result, 1651);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "1947");
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = get_max_release_with_elephant(input);

        assert_eq!(result, 1707);
    }

    // Calculation takes over an hour in release build!
    // #[test]
    // fn part2_input() {
    //     let result = get_solution_part2();

    //     assert_eq!(result, "2556");
    // }
}
