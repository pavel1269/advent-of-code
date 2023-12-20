use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = get_input();
    let result_part1 = part1(&input);
    println!("Part1: {}", result_part1);
    let result_part2 = part2(&input);
    println!("Part2: {}", result_part2);
}

fn part1(input: &str) -> usize {
    let mut circuit = Circuit::from(input);
    let (times_low, times_high) = circuit.run_till(|cycle, _| cycle == 1000);
    return times_low * times_high;
}

fn part2(input: &str) -> usize {
    let mut circuit = Circuit::from(input);
    let target = "rx".to_string();
    let pre_targets: Vec<_> = circuit
        .components
        .iter()
        .filter(|c| c.targets.contains(&target))
        .collect();
    assert!(pre_targets.len() == 1);
    let pre_target_name = pre_targets.get(0).unwrap().name.clone();
    let mut cycles: HashMap<String, Option<usize>> = HashMap::new();
    circuit
        .components
        .iter()
        .filter(|c| c.targets.contains(&pre_target_name))
        .map(|c| c.name.clone())
        .for_each(|target| {
            cycles.insert(target, None);
        });

    let _ = circuit.run_till(|cycle, ongoing_signal| {
        if ongoing_signal.to == pre_target_name && ongoing_signal.signal == Signal::High {
            if let None = cycles.get(&ongoing_signal.from).unwrap() {
                *cycles.get_mut(&ongoing_signal.from).unwrap() = Some(cycle + 1);
            }
        }
        return cycles.values().all(|v| v.is_some());
    });

    let result = cycles
        .values()
        .copied()
        .reduce(|a, b| Some(num::integer::lcm(a.unwrap(), b.unwrap())))
        .unwrap();
    return result.unwrap();
}

#[derive(Debug, Clone)]
struct Circuit {
    components: Vec<Component>,
}

#[derive(Debug, Clone)]
struct OngoinSignal {
    from: String,
    to: String,
    signal: Signal,
}

impl Circuit {
    fn run_till(
        &mut self,
        mut condition: impl FnMut(usize, &OngoinSignal) -> bool,
    ) -> (usize, usize) {
        let mut cycle = 0;
        let mut signal_count_low = 0;
        let mut signal_count_high = 0;
        let mut end = false;
        while !end {
            let from = self
                .components
                .iter()
                .filter(|c| c.ctype == ComponentType::Broadcaster)
                .next()
                .unwrap()
                .name
                .clone();
            let mut signals: VecDeque<_> = VecDeque::from(vec![OngoinSignal {
                from: Component::NAME_SELF.to_string(),
                to: from,
                signal: Signal::Low,
            }]);

            while let Some(ongoing_signal) = signals.pop_front() {
                end = condition(cycle, &ongoing_signal);
                if end {
                    break;
                }

                match ongoing_signal.signal {
                    Signal::Low => signal_count_low += 1,
                    Signal::High => signal_count_high += 1,
                };

                let component = self
                    .components
                    .iter_mut()
                    .filter(|c| c.name == ongoing_signal.to)
                    .next()
                    .unwrap();
                let signal = component.run(ongoing_signal.signal, &ongoing_signal.from);
                if let Some(signal) = signal {
                    let mut signals_new = component
                        .targets
                        .iter()
                        .map(|to| OngoinSignal {
                            signal,
                            to: to.clone(),
                            from: component.name.clone(),
                        })
                        .collect();
                    signals.append(&mut signals_new);
                }
            }

            cycle += 1;
        }

        return (signal_count_low, signal_count_high);
    }

    fn from(str: &str) -> Self {
        let components: Vec<_> = str.lines().map(|line| Component::from(line)).collect();
        assert_eq!(
            1,
            components
                .iter()
                .filter(|c| c.ctype == ComponentType::Broadcaster)
                .count()
        );

        let mut result = Self { components };
        result.init_states();
        return result;
    }

    fn init_states(&mut self) {
        self.init_outputs();
        self.init_states_flipflops();
        self.init_states_conjunctions();
    }

    fn init_outputs(&mut self) {
        let mut targets = self
            .components
            .iter()
            .flat_map(|c| c.targets.clone())
            .collect::<HashSet<_>>()
            .iter()
            .filter(|name| !self.components.iter().any(|c| &&c.name == name))
            .map(|target| Component {
                name: target.clone(),
                ctype: ComponentType::Output,
                memory: HashMap::new(),
                targets: Vec::new(),
            })
            .collect();
        self.components.append(&mut targets);
    }

    fn init_states_flipflops(&mut self) {
        for component_flipflop in self
            .components
            .iter_mut()
            .filter(|c| c.ctype == ComponentType::FlipFlop)
        {
            component_flipflop
                .memory
                .insert(Component::NAME_SELF.to_string(), Signal::Low);
        }
    }

    fn init_states_conjunctions(&mut self) {
        let mut connections = HashMap::new();
        for (index, component_conjunction) in self
            .components
            .iter()
            .enumerate()
            .filter(|(_, c)| c.ctype == ComponentType::Conjunction)
        {
            let connection_names: Vec<_> = self
                .components
                .iter()
                .filter(|c| c.targets.contains(&component_conjunction.name))
                .map(|c| c.name.clone())
                .collect();
            connections.insert(index, connection_names);
        }
        for (index, connection_names) in connections.into_iter() {
            let component = self.components.get_mut(index).unwrap();
            connection_names.into_iter().for_each(|name| {
                component.memory.insert(name.clone(), Signal::Low);
            });
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ComponentType {
    Output,
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone)]
struct Component {
    name: String,
    ctype: ComponentType,
    memory: HashMap<String, Signal>,
    targets: Vec<String>,
}

impl Component {
    const NAME_SELF: &'static str = "";

    fn run(&mut self, signal: Signal, from: &String) -> Option<Signal> {
        match self.ctype {
            ComponentType::Output => None,
            ComponentType::Broadcaster => Some(signal),
            ComponentType::FlipFlop => match signal {
                Signal::Low => {
                    let memory = self.memory.values_mut().next().unwrap();
                    *memory = match memory {
                        Signal::Low => Signal::High,
                        Signal::High => Signal::Low,
                    };
                    return Some(*memory);
                }
                Signal::High => None,
            },
            ComponentType::Conjunction => {
                *self.memory.get_mut(from).unwrap() = signal;
                let is_all_high = self.memory.values().all(|s| s == &Signal::High);
                let signal = if is_all_high {
                    Signal::Low
                } else {
                    Signal::High
                };
                return Some(signal);
            }
        }
    }

    fn from(str: &str) -> Self {
        let regex = regex::Regex::new(r"^([%&]?)([a-zA-Z]+) -> (.+)$").unwrap();
        let captures = regex.captures(str).unwrap();
        let ctype = match captures[1].chars().next() {
            None => ComponentType::Broadcaster,
            Some(char) => match char {
                '%' => ComponentType::FlipFlop,
                '&' => ComponentType::Conjunction,
                _ => panic!(),
            },
        };
        let name = captures[2].to_string();
        let output = captures[3]
            .trim()
            .split(',')
            .map(|str| str.trim().to_string())
            .collect();

        let result = Self {
            ctype,
            name,
            targets: output,
            memory: HashMap::new(),
        };
        return result;
    }
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example1_input() -> &'static str {
        include_str!("./example1.txt")
    }

    fn get_example2_input() -> &'static str {
        include_str!("./example2.txt")
    }

    #[test]
    fn part1_example1() {
        let input = get_example1_input();
        let result = part1(&input);
        assert_eq!(result, 32000000);
    }

    #[test]
    fn part1_example2() {
        let input = get_example2_input();
        let result = part1(&input);
        assert_eq!(result, 11687500);
    }
}
