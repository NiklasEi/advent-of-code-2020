use std::collections::{HashMap, HashSet};
use std::path::Prefix::Verbatim;

fn multiply_corner_image_ids(input: Vec<String>) -> Option<usize> {
    let mut tiles: TileMap = parse_tiles(input);
    let (corners, _borders, _central) = sort_tiles(&mut tiles);

    if corners.len() == 4 {
        return Some(corners.iter().fold(1, |acc, id| acc * id));
    }

    None
}

fn calculate_water_roughness_for_sea_monsters(input: Vec<String>) -> usize {
    let mut tiles: TileMap = parse_tiles(input);
    let puzzle: Vec<Vec<Pixel>> = build_puzzle(&mut tiles);
    let sea_monsters: usize = count_sea_monsters(puzzle.clone());

    puzzle
        .iter()
        .map(|row| {
            row.iter().fold(
                0,
                |acc, pixel| {
                    if pixel == &Pixel::Hash {
                        acc + 1
                    } else {
                        acc
                    }
                },
            )
        })
        .fold(0, |acc, count| acc + count)
        - sea_monsters * 15
}

fn build_puzzle(tiles: &mut TileMap) -> Vec<Vec<Pixel>> {
    let (corners, borders, central) = sort_tiles(tiles);
    const PUZZLE_SIZE: usize = 12;
    let mut puzzle_ids: [[Option<Tile>; PUZZLE_SIZE]; PUZZLE_SIZE] = [
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
    ];

    println!("{:?}", tiles.get(corners.first().unwrap()).unwrap());

    for row in 0..PUZZLE_SIZE {
        for column in 0..PUZZLE_SIZE {
            match (row, column) {
                (0, 0) => {
                    let mut tile = tiles.get(corners.iter().next().unwrap()).unwrap().clone();

                    puzzle_ids[row][column] = Some(tile);
                }
                (_, _) => (),
            }
        }
    }

    let mut puzzle = vec![];

    puzzle
}

fn count_sea_monsters(puzzle: Vec<Vec<Pixel>>) -> usize {
    0
}

fn sort_tiles(tiles: &mut TileMap) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut corners: Vec<usize> = vec![];
    let mut borders: Vec<usize> = vec![];
    let mut central: Vec<usize> = vec![];
    let tiles_clone = tiles.clone();
    let mut tile_ids: Vec<&usize> = tiles_clone.keys().collect();
    while !tile_ids.is_empty() {
        let starting_id = tile_ids.pop().unwrap();
        let mut other_tiles = tiles.clone();
        let start = other_tiles.remove(starting_id).unwrap();
        let mut matched_borders = 0;
        'border: for (direction, border) in start.get_borders() {
            for id in other_tiles.keys().collect::<Vec<&usize>>() {
                if other_tiles
                    .get(id)
                    .unwrap()
                    .get_borders_plus_reversed()
                    .contains(&border)
                {
                    matched_borders += 1;
                    let mut tile = tiles.get_mut(starting_id).unwrap().clone();
                    tile.matches.insert(direction, id.clone());
                    tiles.insert(starting_id.clone(), tile);
                    continue 'border;
                }
            }
        }

        if matched_borders == 2 {
            corners.push(starting_id.clone())
        } else if matched_borders == 3 {
            borders.push(starting_id.clone());
        } else if matched_borders == 4 {
            central.push(starting_id.clone());
        } else {
            panic!("Can't handle amount of border matches")
        }
    }

    (corners, borders, central)
}

type TileMap = HashMap<usize, Tile>;

#[derive(Debug, Clone)]
struct Tile {
    image: Vec<Vec<Pixel>>,
    matches: HashMap<Border, usize>
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Border {
    Top,
    Right,
    Bottom,
    Left,
}

impl Border {
    fn rotate_anti_clock(self)-> Self {
        match self {
            Border::Top => Border::Left,
            Border::Left => Border::Bottom,
            Border::Bottom => Border::Right,
            Border::Right => Border::Top
        }
    }
}

impl Tile {
    fn get_borders(&self) -> Vec<(Border, Vec<Pixel>)> {
        let mut borders: Vec<(Border, Vec<Pixel>)> = vec![];
        borders.push((Border::Top, self.image.first().unwrap().clone()));
        borders.push((Border::Bottom, self.image.last().unwrap().clone()));
        let mut left = vec![];
        let mut right = vec![];

        for row in self.image.iter() {
            left.push(row.first().unwrap().clone());
            right.push(row.last().unwrap().clone());
        }
        borders.push((Border::Right, right));
        borders.push((Border::Left, left));
        borders
    }

    fn get_borders_plus_reversed(&self) -> Vec<Vec<Pixel>> {
        let mut borders: Vec<Vec<Pixel>> = self.get_borders().drain(..).map(|(_direction, border)| border).collect();
        for mut border in borders.clone() {
            border.reverse();
            borders.push(border);
        }
        borders
    }

    fn rotate_anti_clock(&mut self) {
        let old_image = self.image.clone();
        let transformed_image = vec![];
        for row in old_image
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Pixel {
    Dot,
    Hash,
}

fn get_neighbors(row: usize, column: usize) -> Vec<Border> {
    match (row, column) {
        (0, 0) => vec![Border::Right, Border::Bottom],
        (0, 11) => vec![Border::Left, Border::Bottom],
        (11, 11) => vec![Border::Left, Border::Top],
        (11, 0) => vec![Border::Right, Border::Top],
        (0,_) => vec![Border::Left, Border::Bottom, Border::Right],
        (_, 11) => vec![Border::Top, Border::Left, Border::Bottom],
        (11, _) => vec![Border::Right, Border::Top, Border::Left],
        (_,0) => vec![Border::Top, Border::Right, Border::Bottom],
        (_, _) => vec![Border::Top, Border::Right, Border::Bottom, Border::Left]
    }
}

fn parse_tiles(input: Vec<String>) -> TileMap {
    let mut tiles: TileMap = HashMap::new();
    let mut input = input.iter();
    let mut line = input.next();

    while line.is_some() {
        let current_line = line.unwrap();
        if current_line.starts_with("Tile") {
            let id: usize = current_line
                .split(" ")
                .last()
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse()
                .unwrap();
            let mut tile = Tile { image: vec![], matches: HashMap::default() };
            line = input.next();
            let mut current_row;
            while line.is_some() {
                current_row = line.unwrap();
                if current_row == "" {
                    break;
                }
                let mut row = vec![];
                for pixel in current_row.chars() {
                    match pixel {
                        '.' => row.push(Pixel::Dot),
                        '#' => row.push(Pixel::Hash),
                        _ => panic!("unknown pixel"),
                    }
                }
                tile.image.push(row);
                line = input.next();
            }
            tiles.insert(id, tile);
            line = input.next();
        } else {
            break;
        }
    }

    tiles
}

#[cfg(test)]
mod solve {
    use crate::puzzle_20::{calculate_water_roughness_for_sea_monsters, multiply_corner_image_ids};
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_20_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_20.txt");
        println!(
            "Multiple of the corner image IDs: {}",
            multiply_corner_image_ids(input).unwrap()
        );
    }

    #[test]
    fn day_20_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_20.txt");
        println!(
            "Water roughness for sea monsters: {}",
            calculate_water_roughness_for_sea_monsters(input)
        );
    }
}
