fn find_two_numbers_adding_up_to_2020(input: Vec<i32>) -> Option<(i32, i32)> {
    for i in 0..input.len() {
        for k in (i + 1)..input.len() {
            if input[i] + input[k] == 2020 {
                return Some((input[i], input[k]));
            }
        }
    }
    return None;
}

fn find_three_numbers_adding_up_to_2020(input: Vec<i32>) -> Option<(i32, i32, i32)> {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            for k in (j + 1)..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return Some((input[i], input[j], input[k]));
                }
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::puzzle_01::{
        find_three_numbers_adding_up_to_2020, find_two_numbers_adding_up_to_2020,
    };
    use crate::read_file::read_file_to_vec;

    #[test]
    fn solve_day_1_1() {
        let input = read_file_to_vec("input/puzzle_01.txt")
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();
        let answer = find_two_numbers_adding_up_to_2020(input)
            .expect("Unable to find two numbers summing up to 2020");
        println!("{} x {} = {}", answer.0, answer.1, answer.0 * answer.1);
    }

    #[test]
    fn solve_day_1_2() {
        let input = read_file_to_vec("input/puzzle_01.txt")
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();
        let answer = find_three_numbers_adding_up_to_2020(input)
            .expect("Unable to find three numbers summing up to 2020");
        println!(
            "{} x {} x {} = {}",
            answer.0,
            answer.1,
            answer.2,
            answer.0 * answer.1 * answer.2
        );
    }
}
