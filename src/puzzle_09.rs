fn find_first_non_valid_number(input: Vec<String>) -> i64 {
    let numbers: Vec<i64> = parse_input(input);
    for index in 25..numbers.len() {
        if !can_be_combined_from_two_numbers_in_the_previous_n_entries(&numbers, index, 25) {
            return *numbers.iter().nth(index).unwrap();
        }
    }
    *numbers.first().unwrap()
}

fn find_encryption_weakness(input: Vec<String>) -> i64 {
    let numbers: Vec<i64> = parse_input(input.clone());
    let total = find_first_non_valid_number(input);
    let mut components = find_components_of_number(total, numbers).unwrap();
    components.sort();
    let largest = components.pop().unwrap();
    components.reverse();
    let smallest = components.pop().unwrap();

    smallest + largest
}

fn find_components_of_number(final_total: i64, numbers: Vec<i64>) -> Option<Vec<i64>> {
    let mut values: Vec<i64> = vec![];

    for number in numbers {
        values.push(number.clone());
        let mut total: i64 = values.iter().fold(0, |acc, element| acc + element);
        while total > final_total {
            values.reverse();
            let removed = values.pop().unwrap();
            values.reverse();
            total = total - removed;
        }
        if total == final_total {
            return Some(values);
        }
    }
    None
}

fn can_be_combined_from_two_numbers_in_the_previous_n_entries(
    numbers: &Vec<i64>,
    entry_index: usize,
    previous_n: usize,
) -> bool {
    let entry = numbers.iter().nth(entry_index).unwrap();

    let mut previous = numbers[entry_index - previous_n..entry_index].to_vec();
    while previous.len() > 1 {
        let first = previous.pop().unwrap();
        if previous.contains(&(entry - first)) {
            return true;
        }
    }
    false
}

fn parse_input(input: Vec<String>) -> Vec<i64> {
    input
        .iter()
        .map(|string| string.parse::<i64>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::puzzle_09::{find_encryption_weakness, find_first_non_valid_number};
    use crate::read_file::read_file_to_vec;

    #[test]
    fn solve_day_09_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_09.txt");
        println!(
            "First invalid value: {:?}",
            find_first_non_valid_number(input)
        );
    }

    #[test]
    fn solve_day_09_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_09.txt");
        println!("Encryption weakness: {:?}", find_encryption_weakness(input));
    }
}
