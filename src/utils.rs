use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_to_vec(path: &str) -> Vec<String> {
    let mut entries = vec![];
    let file = File::open(path).expect("Unable to open file");
    for line_result in BufReader::new(file).lines() {
        if let Ok(line) = line_result {
            entries.push(line);
        }
    }
    entries
}

#[derive(Debug)]
pub struct Position {
    pub column: usize,
    pub row: usize,
}

pub fn get_neighbor_positions(position: Position, width: usize, height: usize) -> Vec<Position> {
    return match (position.column, position.row) {
        (0, 0) => vec![
            Position { column: 1, row: 0 },
            Position { column: 0, row: 1 },
            Position { column: 1, row: 1 },
        ],
        (column, 0) if column == width - 1 => vec![
            Position {
                column: column - 1,
                row: 0,
            },
            Position {
                column: column - 1,
                row: 1,
            },
            Position { column, row: 1 },
        ],
        (column, row) if column == width - 1 && row == height - 1 => vec![
            Position {
                column: column - 1,
                row: row - 1,
            },
            Position {
                column,
                row: row - 1,
            },
            Position {
                column: column - 1,
                row,
            },
        ],
        (0, row) if row == height - 1 => vec![
            Position {
                column: 0,
                row: row - 1,
            },
            Position {
                column: 1,
                row: row - 1,
            },
            Position { column: 1, row },
        ],
        (column, 0) => vec![
            Position {
                column: column - 1,
                row: 0,
            },
            Position {
                column: column + 1,
                row: 0,
            },
            Position {
                column: column + 1,
                row: 1,
            },
            Position { column, row: 1 },
            Position {
                column: column - 1,
                row: 1,
            },
        ],
        (column, row) if column == width - 1 => vec![
            Position {
                column: column - 1,
                row: row - 1,
            },
            Position {
                column,
                row: row - 1,
            },
            Position {
                column,
                row: row + 1,
            },
            Position {
                column: column - 1,
                row: row + 1,
            },
            Position {
                column: column - 1,
                row,
            },
        ],
        (column, row) if row == height - 1 => vec![
            Position {
                column: column - 1,
                row: row - 1,
            },
            Position {
                column,
                row: row - 1,
            },
            Position {
                column: column + 1,
                row: row - 1,
            },
            Position {
                column: column + 1,
                row,
            },
            Position {
                column: column - 1,
                row,
            },
        ],
        (0, row) => vec![
            Position {
                column: 0,
                row: row - 1,
            },
            Position {
                column: 1,
                row: row - 1,
            },
            Position { column: 1, row },
            Position {
                column: 1,
                row: row + 1,
            },
            Position {
                column: 0,
                row: row + 1,
            },
        ],
        (_, _) => vec![
            Position {
                column: position.column - 1,
                row: position.row - 1,
            },
            Position {
                column: position.column,
                row: position.row - 1,
            },
            Position {
                column: position.column + 1,
                row: position.row - 1,
            },
            Position {
                column: position.column + 1,
                row: position.row,
            },
            Position {
                column: position.column + 1,
                row: position.row + 1,
            },
            Position {
                column: position.column,
                row: position.row + 1,
            },
            Position {
                column: position.column - 1,
                row: position.row + 1,
            },
            Position {
                column: position.column - 1,
                row: position.row,
            },
        ],
    };
}
