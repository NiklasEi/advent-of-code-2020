use std::option::Option::Some;

fn count_bags_in_shiny_gold(input: Vec<String>) -> usize {
    let bags = parse_input(input);
    let shiny_gold = get_bag("shiny gold".to_owned(), &bags).unwrap();
    shiny_gold.count_all_bags(&bags)
}

fn count_bags_that_carry_shiny_gold(input: Vec<String>) -> usize {
    let bags = parse_input(input);
    bags.iter()
        .filter(|bag| bag.carries_shiny_gold(&bags))
        .count()
}

fn parse_input(input: Vec<String>) -> Vec<Bag> {
    let mut bags = vec![];
    for raw_bag in input {
        let mut bag_color_and_contents = raw_bag.split(" bags contain ");
        let mut bag = Bag {
            color: bag_color_and_contents.next().unwrap().clone().to_owned(),
            carries: vec![],
        };
        let contents = bag_color_and_contents
            .next()
            .unwrap()
            .strip_suffix('.')
            .unwrap()
            .split(", ");
        for contain in contents {
            let mut contain_parts = contain.split(" ");
            let amount_str = contain_parts.next().unwrap();
            let amount = amount_str.parse::<usize>();
            if let Ok(amount) = amount {
                let color_part_one = contain_parts.next().unwrap();
                let color_part_two = contain_parts.next().unwrap();
                bag.carries
                    .push((amount, [color_part_one, color_part_two].join(" ")));
            }
        }
        bags.push(bag);
    }
    bags
}

#[derive(Debug)]
struct Bag {
    color: String,
    carries: Vec<(usize, String)>,
}

impl Bag {
    fn carries_shiny_gold(&self, bags: &Vec<Bag>) -> bool {
        if self
            .carries
            .iter()
            .find(|(_amount, color)| color == "shiny gold")
            .is_some()
        {
            return true;
        }
        for bag_color in self.carries.clone() {
            let bag = get_bag(bag_color.1, bags);
            if let Some(bag) = bag {
                if bag.carries_shiny_gold(bags) {
                    return true;
                }
            }
        }
        return false;
    }

    fn count_all_bags(&self, bags: &Vec<Bag>) -> usize {
        if self.carries.len() == 0 {
            return 0;
        }
        let mut count = 0;
        for bag_color in self.carries.clone() {
            let bag = get_bag(bag_color.1, bags);
            if let Some(bag) = bag {
                count += bag_color.0 + bag_color.0 * bag.count_all_bags(bags);
            }
        }
        return count;
    }
}

fn get_bag(bag_color: String, bags: &Vec<Bag>) -> Option<&Bag> {
    bags.iter().find(|&bag| bag.color == bag_color)
}

#[cfg(test)]
mod tests {
    use crate::puzzle_07::{count_bags_in_shiny_gold, count_bags_that_carry_shiny_gold};
    use crate::read_file::read_file_to_vec;

    #[test]
    fn solve_day_07_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_07.txt");

        println!(
            "Number of bags that can carry a shiny gold bag: {:?}",
            count_bags_that_carry_shiny_gold(input)
        );
    }

    #[test]
    fn solve_day_07_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_07.txt");

        println!(
            "Number of bags in shiny gold bag: {:?}",
            count_bags_in_shiny_gold(input)
        );
    }
}
