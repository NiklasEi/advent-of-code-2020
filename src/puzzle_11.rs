use crate::utils::{get_neighbor_positions, Position};

fn occupied_seats_in_stable_situation(input: &mut Vec<String>) -> usize {
    let mut grid: Grid = Grid::from_input(input);
    let mut changed = true;
    while changed {
        changed = grid.frame(Behaviour::Neighbors);
    }

    grid.count_occupied_seats()
}

fn occupied_seats_in_stable_situation_first_in_sight(input: &mut Vec<String>) -> usize {
    let mut grid: Grid = Grid::from_input(input);
    let mut changed = true;
    while changed {
        changed = grid.frame(Behaviour::FirstInSight);
    }

    grid.count_occupied_seats()
}

#[derive(Debug)]
struct Grid {
    seats: Vec<Vec<Slot>>,
    width: usize,
    height: usize,
}

enum Behaviour {
    Neighbors,
    FirstInSight,
}

impl Grid {
    fn from_input(input: &mut Vec<String>) -> Grid {
        let mut grid: Grid = Grid {
            seats: vec![],
            height: input.len(),
            width: input.first().unwrap().len(),
        };
        for line in input.drain(..) {
            grid.seats.push(vec![]);
            let row = grid.seats.last_mut().unwrap();
            for char in line.chars() {
                match char {
                    'L' => row.push(Slot::Seat(false)),
                    '#' => row.push(Slot::Seat(true)),
                    '.' => row.push(Slot::Floor),
                    _ => (),
                }
            }
        }

        grid
    }

    fn frame(&mut self, behaviour: Behaviour) -> bool {
        let max_occupied = match behaviour {
            Behaviour::Neighbors => 3,
            Behaviour::FirstInSight => 4,
        };
        let mut changed = false;
        let mut new_seats: Vec<Vec<Slot>> = vec![];
        for (row, seat_row) in self.seats.iter().enumerate() {
            let mut new_row: Vec<Slot> = vec![];
            for (column, slot) in seat_row.iter().enumerate() {
                match slot {
                    Slot::Seat(occupied) => {
                        let neighbors: Vec<Slot> = match behaviour {
                            Behaviour::Neighbors => {
                                get_neighbor_seats(Position { column, row }, self)
                            }
                            Behaviour::FirstInSight => {
                                get_first_seats_in_sight(Position { column, row }, self)
                            }
                        };
                        let occupied_neighbors = neighbors.iter().fold(0, |count, seat| {
                            if let &Slot::Seat(occ) = seat {
                                if occ {
                                    return count + 1;
                                }
                            }
                            count
                        });
                        if *occupied {
                            if occupied_neighbors > max_occupied {
                                changed = true;
                                new_row.push(Slot::Seat(false));
                            } else {
                                new_row.push(Slot::Seat(true));
                            }
                        } else {
                            if occupied_neighbors == 0 {
                                changed = true;
                                new_row.push(Slot::Seat(true));
                            } else {
                                new_row.push(Slot::Seat(false));
                            }
                        }
                    }
                    Slot::Floor => new_row.push(Slot::Floor),
                }
            }
            new_seats.push(new_row);
        }
        self.seats = new_seats;
        changed
    }

    fn count_occupied_seats(&self) -> usize {
        self.seats.iter().fold(0, |count, row| {
            return count
                + row.iter().fold(0, |seat_count, seat| {
                    if let Slot::Seat(occ) = seat {
                        if *occ {
                            return seat_count + 1;
                        }
                    }
                    seat_count
                });
        })
    }
}

#[derive(Debug, Clone)]
enum Slot {
    Floor,
    Seat(bool),
}

#[derive(Debug)]
struct Direction {
    column: i64,
    row: i64,
}

fn get_first_seats_in_sight(position: Position, grid: &Grid) -> Vec<Slot> {
    let directions = vec![
        Direction {
            column: -1,
            row: -1,
        },
        Direction { column: 0, row: -1 },
        Direction { column: 1, row: -1 },
        Direction { column: -1, row: 0 },
        Direction { column: 1, row: 0 },
        Direction { column: -1, row: 1 },
        Direction { column: 0, row: 1 },
        Direction { column: 1, row: 1 },
    ];
    let mut seats: Vec<Option<Slot>> = directions
        .iter()
        .map(|direction| get_first_seat_in_direction(&position, direction, grid))
        .collect();
    let seats: Vec<Slot> = seats
        .drain(..)
        .filter(|seat| seat.is_some())
        .map(|option| option.unwrap())
        .collect();

    seats
}

fn get_first_seat_in_direction(
    position: &Position,
    direction: &Direction,
    grid: &Grid,
) -> Option<Slot> {
    let mut seat: Option<Slot> = None;
    let mut start_position = Position {
        row: (position.row as i64 + direction.row) as usize,
        column: (position.column as i64 + direction.column) as usize,
    };
    while seat.is_none() && start_position.row < grid.height && start_position.column < grid.width {
        let slot = grid
            .seats
            .iter()
            .nth(start_position.row)
            .unwrap()
            .iter()
            .nth(start_position.column)
            .unwrap();
        if let Slot::Seat(occ) = *slot {
            seat = Some(Slot::Seat(occ));
        } else {
            seat = None;
        }

        start_position.column = (start_position.column as i64 + direction.column) as usize;
        start_position.row = (start_position.row as i64 + direction.row) as usize;
    }

    seat
}

fn get_neighbor_seats(position: Position, grid: &Grid) -> Vec<Slot> {
    let mut positions = get_neighbor_positions_from_grid(position, grid);
    let seats: Vec<Slot> = positions
        .drain(..)
        .map(|position| {
            grid.seats
                .iter()
                .nth(position.row)
                .unwrap()
                .iter()
                .nth(position.column)
                .unwrap()
                .clone()
        })
        .collect();

    seats
}

fn get_neighbor_positions_from_grid(position: Position, grid: &Grid) -> Vec<Position> {
    get_neighbor_positions(position, grid.width, grid.height)
}

#[cfg(test)]
mod solve {
    use crate::puzzle_11::{
        occupied_seats_in_stable_situation, occupied_seats_in_stable_situation_first_in_sight,
    };
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_11_1() {
        let mut input: Vec<String> = read_file_to_vec("input/puzzle_11.txt");
        println!(
            "Occupied seats in stable situation (looking at neighbors): {}",
            occupied_seats_in_stable_situation(&mut input)
        );
    }

    #[test]
    fn day_11_2() {
        let mut input: Vec<String> = read_file_to_vec("input/puzzle_11.txt");
        println!(
            "Occupied seats in stable situation (looking at fist seat in sight): {}",
            occupied_seats_in_stable_situation_first_in_sight(&mut input)
        );
    }
}
