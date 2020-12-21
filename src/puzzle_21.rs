use std::collections::{HashMap, HashSet};

fn occurrences_of_ingredients_without_allergen(input: Vec<String>) -> usize {
    let food_list = parse_food_list(input);
    let allergens_options: HashMap<String, Vec<String>> = get_allergen_options(&food_list);
    let mut all_ingredients: HashSet<String> = HashSet::default();
    for food in food_list.iter() {
        for ingredient in food.ingredients.iter() {
            all_ingredients.insert(ingredient.clone());
        }
    }

    for (_allergen, options) in allergens_options.iter() {
        for option in options {
            all_ingredients.remove(option);
        }
    }

    food_list.iter().fold(0, |acc, food| {
        food.ingredients.iter().fold(0, |acc, ingredient| {
            if all_ingredients.contains(ingredient) {
                acc + 1
            } else {
                acc
            }
        }) + acc
    })
}

fn canonical_dangerous_ingredient_list(input: Vec<String>) -> String {
    let food_list = parse_food_list(input);
    let mut allergens_options: HashMap<String, Vec<String>> = get_allergen_options(&food_list);
    let mut allergens_matches: Vec<(String, String)> = vec![];
    let mut change = true;
    while change {
        change = false;
        for (allergen, options) in allergens_options.clone().iter() {
            if options.len() == 1 {
                change = true;
                let ingredient_match = options.first().unwrap();
                allergens_matches.push((allergen.clone(), ingredient_match.clone()));
                allergens_options.remove(allergen);
                allergens_options
                    .iter_mut()
                    .for_each(|(_allergen, options)| {
                        if let Some(pos) =
                            options.iter().position(|option| option == ingredient_match)
                        {
                            options.remove(pos);
                        }
                    })
            }
        }
    }
    allergens_matches.sort_by(|a, b| a.0.cmp(&b.0));
    allergens_matches
        .drain(..)
        .map(|(_allergen, ingredient_match)| ingredient_match)
        .collect::<Vec<String>>()
        .join(",")
}

fn get_allergen_options(food_list: &Vec<Food>) -> HashMap<String, Vec<String>> {
    let mut all_allergens: Vec<String> = vec![];
    for food in food_list.iter() {
        for allergen in food.allergens.iter() {
            all_allergens.push(allergen.clone());
        }
    }

    let mut allergens_options: HashMap<String, Vec<String>> = HashMap::default();

    for allergen in all_allergens.iter() {
        for food in food_list.iter() {
            if !food.allergens.contains(allergen) {
                continue;
            }
            let options = allergens_options.get(allergen);
            if let Some(options) = options {
                let reduced_options = options
                    .iter()
                    .filter(|&option| food.ingredients.contains(option))
                    .map(|option| option.clone())
                    .collect();
                allergens_options.insert(allergen.clone(), reduced_options);
            } else {
                allergens_options.insert(allergen.clone(), food.ingredients.clone());
            }
        }
    }

    allergens_options
}

fn parse_food_list(input: Vec<String>) -> Vec<Food> {
    let mut food_list = vec![];
    for line in input {
        let mut food = Food {
            ingredients: vec![],
            allergens: vec![],
        };
        let mut split = line.splitn(2, "(contains ").into_iter();
        let ingredients = split.next().unwrap().trim().split(" ");
        for ingredient in ingredients {
            food.ingredients.push(ingredient.to_owned());
        }
        let allergens = split.next().unwrap().strip_suffix(")").unwrap().split(", ");
        for allergen in allergens {
            food.allergens.push(allergen.to_owned());
        }
        food_list.push(food);
    }

    food_list
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

#[cfg(test)]
mod solve {
    use crate::puzzle_21::{
        canonical_dangerous_ingredient_list, occurrences_of_ingredients_without_allergen,
    };
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_21_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_21.txt");
        println!(
            "Occurrences of ingredients without allergen: {}",
            occurrences_of_ingredients_without_allergen(input)
        );
    }

    #[test]
    fn day_21_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_21.txt");
        println!(
            "Canonical dangerous ingredient list: {}",
            canonical_dangerous_ingredient_list(input)
        );
    }
}
