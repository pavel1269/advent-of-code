
use std::collections::HashMap;
use regex::Regex;

const MY_LUGGAGE: &str = "shiny gold";

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = count_possible_colors_containers(&input);

    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let result = count_containers_inside(&input);

    return result;
}

struct Bag {
    color: String,
    content: Vec<BagContent>,
}

impl std::fmt::Debug for Bag {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list()
            .entries(&self.content)
            .finish()
    }
}

struct BagContent {
    color: String,
    quantity: u32,
}

impl std::fmt::Debug for BagContent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("BagContent")
            .field("color", &self.color)
            .field("quentity", &self.quantity)
            .finish()
    }
}

fn count_containers_inside(input: &str) -> i64 {
    let bags = parse_input(&input);

    let mut queue: Vec<(String, i64)> = vec!((MY_LUGGAGE.to_string(), 1));
    let mut count: i64 = -1;
    while !queue.is_empty() {
        let item = queue.pop().unwrap();
        count += item.1;

        let bag = &bags[&item.0];

        for entry in bag.content.iter() {
            queue.push((entry.color.clone(), item.1 * entry.quantity as i64))
        }
    }

    return count;
}

fn count_possible_colors_containers(input: &str) -> i64 {
    let bags = parse_input(&input);

    let mut possibles: HashMap<String, bool> = HashMap::new();
    let mut queue: Vec<String> = vec!(MY_LUGGAGE.to_string());

    while !queue.is_empty() {
        let color = queue.pop().unwrap();
        let next: Vec<&String> = bags.iter().filter(|entry| {
            entry.1.content.iter().any(|bag| bag.color == color)
        }).map(|entry| &entry.1.color).collect();

        for color in next {
            if !possibles.contains_key(color) {
                possibles.insert(color.clone(), true);
                queue.push(color.clone());
            }
        }
    }

    return possibles.len() as i64;
}

fn parse_input(input: &str) -> HashMap<String, Bag> {
    let mut bags: HashMap<String, Bag> = HashMap::new();
    let color_regex = Regex::new("^(\\d+) ([\\s\\w]+) bag[s]?$").unwrap();
    let line_regex = Regex::new("^([\\s\\w]+) bags contain ([\\s\\w,]+)\\.$").unwrap();
    for line in input.lines() {
        let captures = line_regex.captures(line).unwrap();

        let bag_color = captures[1].to_string();
        if !bags.contains_key(&bag_color) {
            let bag = Bag {
                color: captures[1].to_string(),
                content: vec!(),
            };
            bags.insert(bag_color.clone(), bag);
        }

        let content = captures[2].to_string();
        if content == "no other bags" {
            continue;
        }

        for content in content.split(",") {
            let content = content.trim();
            let captures = color_regex.captures(content).unwrap();

            let count = captures[1].parse::<u32>().unwrap();
            bags.get_mut(&bag_color).unwrap().content.push(BagContent {
                color: captures[2].to_string(),
                quantity: count,
            });
        }
    }

    return bags;
}

fn get_challenge_input() -> &'static str {
    include_str!("./inputs/day07.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
striped gold bags contain 4 muted salmon bags, 1 bright yellow bag, 1 dark plum bag, 4 light maroon bags.
"
    }

    #[test]
    fn example_possible_colors() {
        let input = get_example_input();
        let result = count_possible_colors_containers(&input);

        assert_eq!(4, result);
    }

    #[test]
    fn input_possible_colors() {
        let input = get_challenge_input();
        let result = count_possible_colors_containers(&input);

        assert_eq!(246, result);
    }

    #[test]
    fn example1_count_inside() {
        let input = get_example_input();
        let result = count_containers_inside(&input);

        assert_eq!(32, result);
    }

    fn get_example2_input() -> &'static str {
        "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"
    }

    #[test]
    fn example2_count_inside() {
        let input = get_example2_input();
        let result = count_containers_inside(&input);

        assert_eq!(126, result);
    }

    #[test]
    fn input_count_inside() {
        let input = get_challenge_input();
        let result = count_containers_inside(&input);

        assert_eq!(2976, result);
    }
}
