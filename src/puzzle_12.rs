use std::convert::TryInto;

fn manhattan_distance(input: Vec<String>) -> isize {
    let instructions: Vec<Instruction> =
        input.iter().map(|line| line.try_into().unwrap()).collect();
    let mut ship = Ship::default();
    for instruction in instructions {
        ship.navigate_by(instruction);
    }

    ship.get_manhattan_distance()
}

fn manhattan_distance_using_waypoint(input: Vec<String>) -> isize {
    let instructions: Vec<Instruction> =
        input.iter().map(|line| line.try_into().unwrap()).collect();
    let mut ship = Ship::default();
    for instruction in instructions {
        ship.navigate_with_waypoint(instruction);
    }

    ship.get_manhattan_distance()
}

struct Ship {
    direction: Direction,
    north: isize,
    east: isize,
    waypoint: Waypoint,
}

struct Waypoint {
    north: isize,
    east: isize,
}

enum NavigationMethod {
    Direct,
    Waypoint,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            direction: Direction::E,
            north: 0,
            east: 0,
            waypoint: Waypoint { north: 1, east: 10 },
        }
    }
}

impl Ship {
    fn navigate_by(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::N(distance) => self.north += distance as isize,
            Instruction::E(distance) => self.east += distance as isize,
            Instruction::S(distance) => self.north -= distance as isize,
            Instruction::W(distance) => self.east -= distance as isize,
            Instruction::R(angle) => {
                let right_turns = angle / 90;
                self.direction = self.direction.turn_right_n_times(right_turns);
            }
            Instruction::L(angle) => {
                let left_turns = angle / 90;
                let right_turns = 4 - (left_turns % 4);
                self.direction = self.direction.turn_right_n_times(right_turns);
            }
            Instruction::F(distance) => {
                self.navigate_by(Instruction::from_direction(&self.direction, distance))
            }
        }
    }

    fn navigate_with_waypoint(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::N(distance) => self.waypoint.north += distance as isize,
            Instruction::E(distance) => self.waypoint.east += distance as isize,
            Instruction::S(distance) => self.waypoint.north -= distance as isize,
            Instruction::W(distance) => self.waypoint.east -= distance as isize,
            Instruction::R(angle) => {
                let right_turns = angle / 90;
                for _ in 0..right_turns {
                    self.waypoint = Waypoint {
                        north: -self.waypoint.east,
                        east: self.waypoint.north,
                    };
                }
            }
            Instruction::L(angle) => {
                let left_turns = angle / 90;
                let right_turns = 4 - (left_turns % 4);
                for _ in 0..right_turns {
                    self.waypoint = Waypoint {
                        north: -self.waypoint.east,
                        east: self.waypoint.north,
                    };
                }
            }
            Instruction::F(times) => {
                for _ in 0..times {
                    self.north = self.north + self.waypoint.north;
                    self.east = self.east + self.waypoint.east;
                }
            }
        }
    }

    fn get_manhattan_distance(&self) -> isize {
        self.east.abs() + self.north.abs()
    }
}

#[derive(Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn_right_n_times(&self, turns: usize) -> Direction {
        let effective_turns = turns % 4;
        let mut new_direction = self.clone();
        for _ in 0..effective_turns {
            new_direction = new_direction.turn_right()
        }
        new_direction
    }

    fn turn_right(&self) -> Direction {
        match self {
            &Direction::E => Direction::S,
            &Direction::S => Direction::W,
            &Direction::W => Direction::N,
            &Direction::N => Direction::E,
        }
    }
}

enum Instruction {
    N(usize),
    E(usize),
    S(usize),
    W(usize),
    R(usize),
    L(usize),
    F(usize),
}

impl Instruction {
    fn from_direction(direction: &Direction, distance: usize) -> Instruction {
        match direction {
            Direction::N => Instruction::N(distance),
            Direction::E => Instruction::E(distance),
            Direction::S => Instruction::S(distance),
            Direction::W => Instruction::W(distance),
        }
    }
}

impl TryInto<Instruction> for &String {
    type Error = ();

    fn try_into(self) -> Result<Instruction, Self::Error> {
        let (prefix, number) = self.split_at(1);
        let number = number.parse();
        if number.is_err() {
            return Err(());
        }
        let number = number.unwrap();
        match prefix {
            "N" => Ok(Instruction::N(number)),
            "E" => Ok(Instruction::E(number)),
            "S" => Ok(Instruction::S(number)),
            "W" => Ok(Instruction::W(number)),
            "R" => Ok(Instruction::R(number)),
            "L" => Ok(Instruction::L(number)),
            "F" => Ok(Instruction::F(number)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod solve {
    use crate::puzzle_12::{manhattan_distance, manhattan_distance_using_waypoint};
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_12_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_12.txt");
        println!("Manhattan distance: {}", manhattan_distance(input));
    }

    #[test]
    fn day_12_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_12.txt");
        println!(
            "Manhattan distance using waypoint: {}",
            manhattan_distance_using_waypoint(input)
        );
    }
}
