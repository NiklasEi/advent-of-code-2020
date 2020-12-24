use std::collections::HashMap;

const WHITE: bool = false;
const BLACK: bool = true;

fn get_number_of_black_tiles(input: &[String]) -> usize {
    let mut grid: HexGrid<bool> = HexGrid {
        grid: Default::default(),
    };
    for line in input {
        let coordinate = HexCoordinate::parse_from_input(line);
        grid.flip(coordinate, WHITE);
    }

    grid.count_true()
}

fn get_number_of_black_tiles_after_100_days(input: &[String]) -> usize {
    let mut grid: HexGrid<bool> = HexGrid {
        grid: Default::default(),
    };
    for line in input {
        let coordinate = HexCoordinate::parse_from_input(line);
        grid.flip(coordinate, WHITE);
    }
    for _day in 0..100 {
        grid.round();
    }

    grid.count_true()
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct HexCoordinate {
    x: i8,
    y: i8,
    z: i8,
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl HexCoordinate {
    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::East => {
                self.y = self.y + 1;
                self.z = self.z - 1;
            }
            Direction::SouthEast => {
                self.x = self.x + 1;
                self.z = self.z - 1;
            }
            Direction::SouthWest => {
                self.x = self.x + 1;
                self.y = self.y - 1;
            }
            Direction::West => {
                self.y = self.y - 1;
                self.z = self.z + 1;
            }
            Direction::NorthWest => {
                self.x = self.x - 1;
                self.z = self.z + 1;
            }
            Direction::NorthEast => {
                self.x = self.x - 1;
                self.y = self.y + 1;
            }
        }
    }

    fn get_neighbors(&self) -> Vec<HexCoordinate> {
        let mut neighbors = vec![];
        for direction in vec![
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::West,
            Direction::SouthWest,
            Direction::SouthEast,
            Direction::East,
        ] {
            let mut corrdinate = self.clone();
            corrdinate.go(direction);
            neighbors.push(corrdinate);
        }

        neighbors
    }

    fn parse_from_input(mut line: &str) -> HexCoordinate {
        let mut coordinate = HexCoordinate { x: 0, y: 0, z: 0 };
        while !line.is_empty() {
            if line.starts_with("e") {
                coordinate.go(Direction::East);
                line = line.strip_prefix("e").unwrap();
            } else if line.starts_with("se") {
                coordinate.go(Direction::SouthEast);
                line = line.strip_prefix("se").unwrap();
            } else if line.starts_with("sw") {
                coordinate.go(Direction::SouthWest);
                line = line.strip_prefix("sw").unwrap();
            } else if line.starts_with("w") {
                coordinate.go(Direction::West);
                line = line.strip_prefix("w").unwrap();
            } else if line.starts_with("nw") {
                coordinate.go(Direction::NorthWest);
                line = line.strip_prefix("nw").unwrap();
            } else if line.starts_with("ne") {
                coordinate.go(Direction::NorthEast);
                line = line.strip_prefix("ne").unwrap();
            } else {
                panic!("unknown direction");
            }
        }

        coordinate
    }
}

struct HexGrid<T> {
    grid: HashMap<HexCoordinate, T>,
}

impl<T> HexGrid<T> {
    fn get(&self, coordinate: &HexCoordinate) -> Option<&T> {
        self.grid.get(coordinate)
    }
}

impl HexGrid<bool> {
    fn flip(&mut self, coordinate: HexCoordinate, default: bool) {
        let current = self.get(&coordinate);
        let current = if current.is_none() {
            default
        } else {
            current.unwrap().clone()
        };
        self.grid.insert(coordinate, !current);
    }

    fn round(&mut self) {
        self.prepare_for_round();
        let mut new_grid = self.grid.clone();
        for (coordinate, &status) in self.grid.iter() {
            let neighboring_black_tiles = self.count_neighboring_black_tiles(coordinate);
            if status && (neighboring_black_tiles == 0 || neighboring_black_tiles > 2) {
                new_grid.insert(coordinate.clone(), WHITE);
            } else if !status && neighboring_black_tiles == 2 {
                new_grid.insert(coordinate.clone(), BLACK);
            }
        }
        self.grid = new_grid;
    }

    fn prepare_for_round(&mut self) {
        // we need to set all surrounding tiles of black tiles to white (if they are not jet set)
        for (coordinate, &status) in self.grid.clone().iter() {
            if !status {
                continue;
            }
            let neighbor_coordinates = coordinate.get_neighbors();
            for coordinate in neighbor_coordinates {
                if self.get(&coordinate).is_some() {
                    continue;
                }
                self.grid.insert(coordinate, WHITE);
            }
        }
    }

    fn get_neighbors(&self, coordinate: &HexCoordinate) -> HashMap<HexCoordinate, bool> {
        let neighbor_coordinates = coordinate.get_neighbors();
        let mut neighbors = HashMap::default();
        for coordinate in neighbor_coordinates {
            if let Some(value) = self.get(&coordinate) {
                neighbors.insert(coordinate, value.clone());
            }
        }

        neighbors
    }

    fn count_neighboring_black_tiles(&self, coordinate: &HexCoordinate) -> usize {
        self.get_neighbors(coordinate)
            .values()
            .fold(0, |acc, &current| if current { acc + 1 } else { acc })
    }

    fn count_true(&self) -> usize {
        self.grid
            .values()
            .fold(0, |acc, &current| if current { acc + 1 } else { acc })
    }
}

#[cfg(test)]
mod solve {
    use crate::puzzle_24::{get_number_of_black_tiles, get_number_of_black_tiles_after_100_days};
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_24_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_24.txt");
        println!(
            "Number of black tiles: {}",
            get_number_of_black_tiles(&input)
        );
    }

    #[test]
    fn day_24_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_24.txt");
        println!(
            "Number of black tiles after 100 days: {}",
            get_number_of_black_tiles_after_100_days(&input)
        );
    }
}
