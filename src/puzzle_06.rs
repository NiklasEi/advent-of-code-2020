fn count_questions_to_which_anyone_answered_yes(input: Vec<String>) -> usize {
    let mut count = 0;
    let mut group_answers = parse_answers(input);
    for group in group_answers.iter_mut() {
        group.remove_duplicate_answers();
        count += group.answers.len();
    }

    count
}

fn count_questions_to_which_everyone_answered_yes(input: Vec<String>) -> usize {
    let mut count = 0;
    let mut group_answers = parse_answers(input);
    for group in group_answers.iter_mut() {
        count += group.count_yes_from_all_in_group();
    }

    count
}

fn parse_answers(input: Vec<String>) -> Vec<Group> {
    let mut group_answers = vec![Group::default()];
    for line in input {
        if line == "" {
            group_answers.push(Group::default());
            continue;
        }
        let chars = line.chars();
        let group = group_answers.last_mut().unwrap();
        group.people += 1;
        for char in chars {
            group.answers.push(char);
        }
    }

    group_answers
}

#[derive(Default)]
struct Group {
    people: usize,
    answers: Vec<char>,
}

impl Group {
    fn remove_duplicate_answers(&mut self) {
        self.answers.sort();
        self.answers.dedup();
    }

    fn count_yes_from_all_in_group(&mut self) -> usize {
        let mut yes_from_all = 0;
        self.answers.sort();
        let mut unique_answers = self.answers[..].to_vec();
        unique_answers.dedup();
        for unique_answer in unique_answers {
            let answer_count = self
                .answers
                .iter()
                .filter(|&&answer| answer == unique_answer)
                .count();
            if answer_count == self.people {
                yes_from_all += 1;
            }
        }

        yes_from_all
    }
}

#[cfg(test)]
mod solve {
    use crate::puzzle_06::{
        count_questions_to_which_anyone_answered_yes,
        count_questions_to_which_everyone_answered_yes,
    };
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_06_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_06.txt");

        println!(
            "Number of 'yes' in all groups: {}",
            count_questions_to_which_anyone_answered_yes(input)
        );
    }

    #[test]
    fn day_06_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_06.txt");

        println!(
            "Number of 'yes' from everyone in all groups: {}",
            count_questions_to_which_everyone_answered_yes(input)
        );
    }
}
