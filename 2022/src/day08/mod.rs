
pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = count_visible_trees(input);
    return result.to_string();
}

fn count_visible_trees(input: &str) -> usize {
    let visibility = calculate_visibility(input);
    let mut count = 0;
    for row in visibility.iter() {
        count += row.iter().filter(|v| **v).count();
    }
    return count;
}

fn calculate_visibility(input: &str) -> Vec<Vec<bool>> {
    let tree_map = parse_input(input);
    let height = tree_map.len();
    let width = tree_map.iter().nth(0).unwrap().len();

    let mut visibility = Vec::with_capacity(height);
    for _ in 0..height {
        visibility.push(vec![false; width]);
    }

    // From sides
    for row_index in 0..height {
        let mut highest_tree = 0;
        for column_index in 0..width {
            let current_tree = tree_map[row_index][column_index];
            if current_tree > highest_tree || column_index == 0 {
                highest_tree = current_tree;
                visibility[row_index][column_index] = true;
            }
        }
        
        let mut highest_tree = 0;
        for column_index in (0..width).rev() {
            let current_tree = tree_map[row_index][column_index];
            if current_tree > highest_tree || column_index == width - 1 {
                highest_tree = current_tree;
                visibility[row_index][column_index] = true;
            }
        }
    }

    // From up and down
    for column_index in 0..width {
        let mut highest_tree = 0;
        for row_index in 0..height {
            let current_tree = tree_map[row_index][column_index];
            if current_tree > highest_tree || row_index == 0 {
                highest_tree = current_tree;
                visibility[row_index][column_index] = true;
            }
        }
        
        let mut highest_tree = 0;
        for row_index in (0..height).rev() {
            let current_tree = tree_map[row_index][column_index];
            if current_tree > highest_tree || row_index == height - 1 {
                highest_tree = current_tree;
                visibility[row_index][column_index] = true;
            }
        }
    }

    return visibility;
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().len();

    let mut map = Vec::with_capacity(height);
    for _ in 0..height {
        map.push(Vec::with_capacity(width));
    }

    for (row_index, row) in input.lines().enumerate() {
        let map_row = &mut map[row_index];
        for tree in row.chars() {
            map_row.push(tree.to_digit(10).unwrap());
        }
    }

    return map;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "30373
25512
65332
33549
35390"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = count_visible_trees(input);

        assert_eq!(result, 21);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "1785");
    }
}
