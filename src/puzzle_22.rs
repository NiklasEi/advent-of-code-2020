fn get_points_of_winning_player(input: &[String]) -> usize {
    let mut combat = Combat::parse(input);

    let mut state = combat.play_combat_round();
    while state == GameState::Playing {
        state = combat.play_combat_round();
    }

    combat
        .player_two
        .iter()
        .enumerate()
        .fold(0, |acc, (index, card)| acc + (index + 1) * card)
}

fn get_points_of_winning_player_in_recursive_combat(input: &[String]) -> usize {
    let mut combat = Combat::parse(input);

    let player_one_won = combat.play_recursive_combat_game();

    return if player_one_won {
        combat
            .player_one
            .iter()
            .enumerate()
            .fold(0, |acc, (index, card)| acc + (index + 1) * card)
    } else {
        combat
            .player_two
            .iter()
            .enumerate()
            .fold(0, |acc, (index, card)| acc + (index + 1) * card)
    };
}

#[derive(Debug)]
struct Combat {
    player_one: Vec<usize>,
    player_two: Vec<usize>,
    previous_rounds: Vec<String>,
}

type PlayerOneWon = bool;

impl Combat {
    fn parse(input: &[String]) -> Self {
        let mut combat = Combat {
            player_one: vec![],
            player_two: vec![],
            previous_rounds: vec![],
        };
        let mut input = input.iter();
        let mut line;
        let mut player_one = true;
        loop {
            line = input.next();
            if line.is_none() || line.unwrap() == "" {
                if player_one {
                    player_one = false;
                    continue;
                } else {
                    break;
                }
            }
            let line_value = line.unwrap();
            if line_value.starts_with("Player") {
                continue;
            }
            let value: usize = line_value.parse().unwrap();
            if player_one {
                combat.player_one.push(value);
            } else {
                combat.player_two.push(value);
            }
        }
        combat.player_two.reverse();
        combat.player_one.reverse();

        combat
    }
    fn play_recursive_combat_game(&mut self) -> PlayerOneWon {
        let mut state = self.play_recursive_combat_game_round();
        while state == GameState::Playing {
            state = self.play_recursive_combat_game_round();
        }

        if let GameState::Finished(player_one_won) = state {
            return player_one_won;
        } else {
            panic!("Game did not finish")
        }
    }

    fn play_recursive_combat_game_round(&mut self) -> GameState {
        let current_state = self
            .player_one
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",")
            + ";"
            + &self
                .player_two
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(",");
        if self.previous_rounds.contains(&current_state) {
            return GameState::Finished(true);
        } else {
            self.previous_rounds.push(current_state);
        }
        let player_one = self.player_one.pop();
        if player_one.is_none() {
            return GameState::Finished(false);
        }
        let player_two = self.player_two.pop();
        if player_two.is_none() {
            self.player_one.push(player_one.unwrap());
            return GameState::Finished(true);
        }
        let player_one = player_one.unwrap();
        let player_two = player_two.unwrap();
        let player_one_won =
            if self.player_one.len() >= player_one && self.player_two.len() >= player_two {
                let mut sub_game = Combat {
                    player_one: self.player_one[(self.player_one.len() - player_one)..].to_owned(),
                    player_two: self.player_two[(self.player_two.len() - player_two)..].to_owned(),
                    previous_rounds: vec![],
                };

                sub_game.play_recursive_combat_game()
            } else {
                player_one > player_two
            };

        if player_one_won {
            self.player_one.reverse();
            self.player_one.push(player_one);
            self.player_one.push(player_two);
            self.player_one.reverse();
        } else {
            self.player_two.reverse();
            self.player_two.push(player_two);
            self.player_two.push(player_one);
            self.player_two.reverse();
        }

        GameState::Playing
    }

    fn play_combat_round(&mut self) -> GameState {
        let player_one = self.player_one.pop();
        if player_one.is_none() {
            return GameState::Finished(false);
        }
        let player_two = self.player_two.pop();
        if player_two.is_none() {
            self.player_one.push(player_one.unwrap());
            return GameState::Finished(true);
        }
        let player_one = player_one.unwrap();
        let player_two = player_two.unwrap();
        if player_one > player_two {
            self.player_one.reverse();
            self.player_one.push(player_one);
            self.player_one.push(player_two);
            self.player_one.reverse();
        } else {
            self.player_two.reverse();
            self.player_two.push(player_two);
            self.player_two.push(player_one);
            self.player_two.reverse();
        }

        GameState::Playing
    }
}

#[derive(PartialEq)]
enum GameState {
    Finished(bool),
    Playing,
}

#[cfg(test)]
mod solve {
    use crate::puzzle_22::{
        get_points_of_winning_player, get_points_of_winning_player_in_recursive_combat,
    };
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_22_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_22.txt");
        println!(
            "Points of winning player: {}",
            get_points_of_winning_player(&input)
        );
    }

    #[test]
    fn day_22_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_22.txt");
        println!(
            "Points of winning player (recursive): {}",
            get_points_of_winning_player_in_recursive_combat(&input)
        );
    }
}
