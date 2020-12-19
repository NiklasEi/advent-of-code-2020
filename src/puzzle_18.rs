fn evaluate_homework(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|line| eval(line, false).0)
        .fold(0, |acc, value| value + acc)
}

fn evaluate_homework_with_addition_first(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|line| eval(line, true).0)
        .fold(0, |acc, value| value + acc)
}

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Times,
}

impl Operator {
    fn apply(&self, acc: &mut i64, value: i64) {
        match self {
            Operator::Plus => *acc += value,
            Operator::Times => *acc *= value,
        }
    }
}

fn eval(expr: &str, plus_has_precedence: bool) -> (i64, usize) {
    let mut acc = 0;
    let mut operator: Option<Operator> = None;

    let mut i = 0;

    while i < expr.len() {
        match expr.as_bytes()[i] {
            b' ' => {
                i += 1;
            }
            b'+' => {
                operator = Some(Operator::Plus);
                i += 1;
            }
            b'*' => {
                operator = Some(Operator::Times);
                i += 1;

                if plus_has_precedence {
                    let (value, n) = eval(&expr[i..], plus_has_precedence);
                    i += n;

                    if let Some(op) = operator {
                        op.apply(&mut acc, value);
                    } else {
                        acc = value;
                    }
                }
            }
            b'(' => {
                let (value, n) = eval(&expr[i + 1..], plus_has_precedence);
                // +2 to skip both braces '(' and ')'
                i += n + 2;

                if let Some(op) = operator {
                    op.apply(&mut acc, value);
                } else {
                    acc = value;
                }
            }
            b')' => return (acc, i),
            n if (b'0'..=b'9').contains(&n) => {
                let number = expr[i..].splitn(2, " ").next().unwrap().trim();
                let number = number.replace(")", "");
                i += number.len();

                if let Some(op) = operator {
                    op.apply(&mut acc, number.parse().unwrap());
                } else {
                    acc = number.parse().unwrap();
                }
            }
            _ => panic!("unsupported char {}", expr),
        }
    }

    (acc, i)
}

#[cfg(test)]
mod solve {
    use crate::puzzle_18::{evaluate_homework, evaluate_homework_with_addition_first};
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_18_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_18.txt");
        println!("Evaluated homework: {}", evaluate_homework(input));
    }

    #[test]
    fn day_18_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_18.txt");
        println!(
            "Evaluated homework with addition first: {}",
            evaluate_homework_with_addition_first(input)
        );
    }
}
