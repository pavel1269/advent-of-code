use std::collections::{HashMap, LinkedList};

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = count_part1(input);

    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let result = list_dangerou_ingredients(input);

    println!("Real result: {}", &result);
    return -1;
}

struct Food {
    ingredients: LinkedList<usize>,
    allergens: LinkedList<usize>,
}

struct Menu {
    foods: LinkedList<Food>,
    ingredient_names: Vec<String>,
    allergen_names: Vec<String>,
}

fn count_part1(input: &str) -> i64 {
    let menu = parse_menu(input);
    let ingredient_allergen = identify_allergens(&menu);
    let result = count_ingredients_without_allergen(&menu, &ingredient_allergen);

    return result;
}

fn list_dangerou_ingredients(input: &str) -> String {
    let menu = parse_menu(input);
    let ingredient_allergen = identify_allergens(&menu);

    let mut allergens = menu
        .allergen_names
        .iter()
        .cloned()
        .enumerate()
        .collect::<Vec<(usize, String)>>();
    allergens.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let result = allergens
        .iter()
        .map(|(allergen, _)| {
            let res = ingredient_allergen
                .iter()
                .find(|ingredient_allergen| ingredient_allergen.1 == allergen)
                .unwrap();
            return menu.ingredient_names.get(*res.0).unwrap().clone();
        })
        .collect::<Vec<String>>()
        .join(",");

    return result;
}

fn count_ingredients_without_allergen(
    menu: &Menu,
    ingredient_allergen: &HashMap<usize, usize>,
) -> i64 {
    let known_ingredients = ingredient_allergen.keys().collect::<Vec<&usize>>();
    let ingredinets_without_allergen = (0..menu.ingredient_names.len())
        .filter(|ingredient| !known_ingredients.contains(&ingredient))
        .collect::<Vec<usize>>();

    let mut count: i64 = 0;
    for food in menu.foods.iter() {
        let food_count = food
            .ingredients
            .iter()
            .filter(|ingredient| ingredinets_without_allergen.contains(ingredient))
            .count();
        count += food_count as i64;
    }

    return count;
}

fn identify_allergens(menu: &Menu) -> HashMap<usize, usize> {
    let mut ingredients_per_allergen: HashMap<usize, LinkedList<LinkedList<usize>>> =
        HashMap::new();
    for (alergen_index, _) in menu.allergen_names.iter().enumerate() {
        ingredients_per_allergen.insert(alergen_index, LinkedList::new());
    }

    for food in menu.foods.iter() {
        for allergen in food.allergens.iter() {
            ingredients_per_allergen
                .get_mut(allergen)
                .unwrap()
                .push_back(food.ingredients.clone());
        }
    }

    let mut allergens_to_resolve: Vec<usize> = (0..menu.allergen_names.len()).collect();
    let mut ingredient_allergen: HashMap<usize, usize> = HashMap::new();
    while allergens_to_resolve.len() > 0 {
        for (allergen_index, allergen) in allergens_to_resolve.iter().enumerate() {
            let ingredient_lists = ingredients_per_allergen.get(allergen).unwrap();
            let mut possible_ingredients = ingredient_lists.iter().next().unwrap().clone();
            for others_ingredients in ingredient_lists.iter().skip(1) {
                possible_ingredients = possible_ingredients
                    .iter()
                    .copied()
                    .filter(|ingredient| others_ingredients.contains(ingredient))
                    .collect();
            }

            let known_ingredients = ingredient_allergen.keys().collect::<Vec<&usize>>();
            possible_ingredients = possible_ingredients
                .iter()
                .copied()
                .filter(|possible_ingredient| !known_ingredients.contains(&possible_ingredient))
                .collect();

            if possible_ingredients.len() == 0 {
                panic!();
            } else if possible_ingredients.len() == 1 {
                ingredient_allergen.insert(*possible_ingredients.iter().next().unwrap(), *allergen);
                allergens_to_resolve.remove(allergen_index);
                break;
            }
        }
    }

    return ingredient_allergen;
}

fn parse_menu(input: &str) -> Menu {
    let allergen_parse_skip_size = 9; // (contains
    let mut ingredient_names: Vec<String> = Vec::new();
    let mut allergen_names: Vec<String> = Vec::new();
    let mut foods: LinkedList<Food> = LinkedList::new();
    for mut ingredients in input.lines() {
        let mut allergens: &str = "";
        match ingredients.find('(') {
            Some(index) => {
                allergens =
                    ingredients[index + allergen_parse_skip_size..ingredients.len() - 1].trim();
                ingredients = ingredients[..index].trim();
            }
            None => {}
        }

        let mut ingredient_list = LinkedList::new();
        for ingredient in ingredients.split(' ') {
            let ingredient = ingredient.to_string();
            if !ingredient_names.contains(&ingredient) {
                ingredient_names.push(ingredient.clone());
            }

            let ingredient_index = ingredient_names
                .iter()
                .position(|ingredient_name| ingredient_name == &ingredient)
                .unwrap();
            ingredient_list.push_back(ingredient_index);
        }

        let mut allergen_list = LinkedList::new();
        if allergens.len() > 0 {
            for allergen in allergens.split(',') {
                let allergen = allergen.trim().to_string();
                if !allergen_names.contains(&allergen) {
                    allergen_names.push(allergen.clone());
                }

                let ingredient_index = allergen_names
                    .iter()
                    .position(|ingredient_name| ingredient_name == &allergen)
                    .unwrap();
                allergen_list.push_back(ingredient_index);
            }
        }

        // println!("allergens: {:?}, ingredients: {:?}", allergen_list, ingredient_list);
        foods.push_back(Food {
            ingredients: ingredient_list,
            allergens: allergen_list,
        });
    }

    return Menu {
        foods: foods,
        ingredient_names: ingredient_names,
        allergen_names: allergen_names,
    };
}

fn get_challenge_input() -> &'static str {
    include_str!("./inputs/day21.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_example_input() -> &'static str {
        "mxmxvkd kfcds sqjhc nhms (contains fish, dairy)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
mxmxvkd sqjhc
"
    }

    #[test]
    fn example_count_part1_result() {
        let input = get_example_input();
        let result = count_part1(input);

        assert_eq!(5, result);
    }

    #[test]
    fn input_part1_result() {
        let result = get_part1_result();

        assert_eq!(2659, result);
    }

    #[test]
    fn example_list_ingredients() {
        let input = get_example_input();
        let list = list_dangerou_ingredients(input);

        assert_eq!("mxmxvkd,sqjhc,fvjkl", list);
    }

    #[test]
    fn input_list_ingredients() {
        let input = get_challenge_input();
        let list = list_dangerou_ingredients(input);

        assert_eq!("rcqb,cltx,nrl,qjvvcvz,tsqpn,xhnk,tfqsb,zqzmzl", list);
    }
}
