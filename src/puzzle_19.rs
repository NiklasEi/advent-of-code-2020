use std::collections::HashMap;

fn count_rule_zero_valid_messages(input: Vec<String>) -> usize {
    let (rules, messages) = parse(input);

    messages
        .iter()
        .filter(|&msg| {
            rules
                .get(&0)
                .unwrap()
                .evaluate_possible_matches(&rules, msg)
                .contains(&msg.len())
        })
        .count()
}

fn count_rule_zero_valid_messages_with_loops(input: Vec<String>) -> usize {
    let (mut rules, messages) = parse(input);

    rules.insert(8, Rule::Combine(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Combine(vec![vec![42, 31], vec![42, 11, 31]]));

    messages
        .iter()
        .filter(|&msg| {
            rules
                .get(&0)
                .unwrap()
                .evaluate_possible_matches(&rules, msg)
                .contains(&msg.len())
        })
        .count()
}

fn parse(input: Vec<String>) -> (RuleMap, Vec<String>) {
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let mut lines = input.iter();
    let mut line = lines.next();
    while line.is_some() {
        if !line.unwrap().contains(":") {
            break;
        }
        let mut key_and_rule = line.unwrap().split(": ");
        let key: usize = key_and_rule.next().unwrap().parse().unwrap();
        let raw_rule = key_and_rule.next().unwrap().trim();
        if raw_rule.contains("a") {
            rules.insert(key, Rule::A);
        } else if raw_rule.contains("b") {
            rules.insert(key, Rule::B);
        } else {
            let options = raw_rule.split("|");
            let mut combinations: Vec<Vec<usize>> = vec![];
            for option in options {
                let rule_keys = option.trim().split(" ");
                combinations.push(rule_keys.map(|key| key.parse::<usize>().unwrap()).collect());
            }
            rules.insert(key, Rule::Combine(combinations));
        }
        line = lines.next();
    }

    line = lines.next();
    let mut messages = vec![];
    while line.is_some() {
        messages.push(line.unwrap().trim().to_owned());
        line = lines.next();
    }

    (rules, messages)
}

type RuleMap = HashMap<usize, Rule>;

#[derive(PartialEq, Debug)]
enum Rule {
    A,
    B,
    Combine(Vec<Vec<usize>>),
}

impl Rule {
    fn evaluate_possible_matches(&self, rules: &HashMap<usize, Rule>, s: &str) -> Vec<usize> {
        if s.is_empty() {
            return vec![];
        }

        match self {
            Rule::A => {
                if s.starts_with("a") {
                    vec![1]
                } else {
                    vec![]
                }
            }
            Rule::B => {
                if s.starts_with("b") {
                    vec![1]
                } else {
                    vec![]
                }
            }
            Rule::Combine(options) => options
                .iter()
                .flat_map(|options| {
                    let mut possible_n = vec![0];

                    for rule in options.iter().map(|rule_is| rules.get(rule_is).unwrap()) {
                        possible_n = possible_n
                            .into_iter()
                            .flat_map(|n| {
                                rule.evaluate_possible_matches(rules, &s[n..])
                                    .into_iter()
                                    .map(move |result| n + result)
                            })
                            .collect();
                    }

                    possible_n
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod solve {
    use crate::puzzle_19::{
        count_rule_zero_valid_messages, count_rule_zero_valid_messages_with_loops,
    };
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_19_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_19.txt");
        println!(
            "Messages valid for rule 0: {}",
            count_rule_zero_valid_messages(input)
        );
    }

    #[test]
    fn day_19_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_19.txt");
        println!(
            "Messages valid for rule 0: {}",
            count_rule_zero_valid_messages_with_loops(input)
        );
    }
}
