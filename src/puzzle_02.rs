fn count_valid_passwords_for_sled_rental_rules(input: Vec<PasswordAndRule>) -> usize {
    input
        .iter()
        .filter(|password_and_rule| -> bool {
            let count = password_and_rule
                .3
                .chars()
                .filter(|char| char == &password_and_rule.2)
                .count();
            count >= password_and_rule.0 && count <= password_and_rule.1
        })
        .count()
}

fn count_valid_passwords_for_toboggan_rental_rules(input: Vec<PasswordAndRule>) -> usize {
    input
        .iter()
        .filter(|password_and_rule| -> bool {
            let first_char = get_nth_char(&password_and_rule.3, password_and_rule.0);
            let second_char = get_nth_char(&password_and_rule.3, password_and_rule.1);
            if first_char == second_char {
                return false;
            }
            first_char == password_and_rule.2 || second_char == password_and_rule.2
        })
        .count()
}

fn get_nth_char(string: &str, position_starting_at_one: usize) -> char {
    string.chars().nth(position_starting_at_one - 1).unwrap()
}

#[derive(Debug)]
struct PasswordAndRule(usize, usize, char, String);

fn convert_to_password_and_rule(line: &str) -> PasswordAndRule {
    let parts: Vec<&str> = line.split(" ").collect();
    let range: Vec<&str> = parts[0].split("-").collect();
    let range_beginning = range[0].parse::<usize>().unwrap();
    let range_end = range[1].parse::<usize>().unwrap();
    let char = parts[1].replace(":", "").chars().collect::<Vec<char>>()[0];
    let password = parts[2];
    PasswordAndRule(range_beginning, range_end, char, password.to_owned())
}

#[cfg(test)]
mod solve {
    use crate::puzzle_02::{
        convert_to_password_and_rule, count_valid_passwords_for_sled_rental_rules,
        count_valid_passwords_for_toboggan_rental_rules, PasswordAndRule,
    };
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_02_1() {
        let input: Vec<PasswordAndRule> = read_file_to_vec("input/puzzle_02.txt")
            .iter()
            .map(|rule| convert_to_password_and_rule(rule))
            .collect();
        println!(
            "There are {} valid passwords",
            count_valid_passwords_for_sled_rental_rules(input)
        );
    }

    #[test]
    fn day_02_2() {
        let input: Vec<PasswordAndRule> = read_file_to_vec("input/puzzle_02.txt")
            .iter()
            .map(|rule| convert_to_password_and_rule(rule))
            .collect();
        println!(
            "There are {} valid passwords",
            count_valid_passwords_for_toboggan_rental_rules(input)
        );
    }
}
