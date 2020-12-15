use std::collections::HashMap;

fn find_nth_number_spoken(input: Vec<String>, turn: usize) -> usize {
    let starting_numbers: Vec<usize> = input
        .first()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    let mut game = Game {
        turn: 0,
        previous_turns: HashMap::default(),
        last_number: 0,
    };
    for starting_number in starting_numbers {
        game.say_number(starting_number);
    }

    while game.turn < turn - 1 {
        game.next_number();
    }
    game.next_number()
}

struct Game {
    turn: usize,
    previous_turns: HashMap<usize, Vec<usize>>,
    last_number: usize,
}

impl Game {
    fn say_number(&mut self, number: usize) {
        let mut turns = self
            .previous_turns
            .get_mut(&number)
            .unwrap_or(&mut vec![])
            .clone();
        turns.reverse();
        while turns.len() > 1 {
            turns.pop();
        }
        turns.reverse();
        turns.push(self.turn);
        self.previous_turns.insert(number, turns);
        self.turn += 1;
        self.last_number = number;
    }

    fn next_number(&mut self) -> usize {
        let turns_for_last_number = self.previous_turns.get(&self.last_number).unwrap();
        if turns_for_last_number.len() == 1 {
            self.say_number(0);
            return 0;
        }

        if turns_for_last_number.len() != 2 {
            println!("found {:?}", turns_for_last_number);
            panic!("too many saved turns")
        }

        let diff = turns_for_last_number.last().unwrap() - turns_for_last_number.first().unwrap();
        self.say_number(diff);
        diff
    }
}

#[cfg(test)]
mod solve {
    use crate::puzzle_15::find_nth_number_spoken;
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_15_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_15.txt");
        println!(
            "2020th number spoken: {}",
            find_nth_number_spoken(input, 2020)
        );
    }

    #[test]
    fn day_15_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_15.txt");
        println!(
            "30 000 000th number spoken: {}",
            find_nth_number_spoken(input, 30_000_000)
        );
    }
}
