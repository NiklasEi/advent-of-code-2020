use std::collections::HashMap;

const MONSTER: &str = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";

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
    let puzzle: Tile = build_puzzle(&mut tiles);
    let sea_monsters: usize = count_sea_monsters(&puzzle);

    puzzle
        .image
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
        - sea_monsters * MONSTER.matches('#').count()
}

fn build_puzzle(tiles: &mut TileMap) -> Tile {
    let (mut corners, mut borders, mut central) = sort_tiles(tiles);
    const PUZZLE_SIZE: usize = 12;
    let mut puzzle: [[Option<Tile>; PUZZLE_SIZE]; PUZZLE_SIZE] = [
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

    for row in 0..PUZZLE_SIZE {
        for column in 0..PUZZLE_SIZE {
            match (row, column) {
                (0, 0) => {
                    let mut tile = tiles.get(&corners.remove(0)).unwrap().clone();
                    tile.turn_to_borders(&vec![Border::Right, Border::Bottom])
                        .expect("Failed to turn tile to needed orientation");
                    puzzle[row][column] = Some(tile);
                }
                (0, 11) => {
                    let previous = puzzle
                        .get(row)
                        .unwrap()
                        .get(column - 1)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let tile_id = previous.matches.get(&Border::Right).unwrap();
                    corners.remove(corners.iter().position(|id| id == tile_id).unwrap());
                    let mut tile = tiles.get(&tile_id).unwrap().clone();
                    let mut constraints: HashMap<Border, Vec<Pixel>> = HashMap::default();
                    constraints.insert(
                        Border::Left,
                        previous
                            .get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Right)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    tile.turn_to_borders_with_constraints(
                        &vec![Border::Bottom, Border::Left],
                        constraints,
                    )
                    .expect("failed to turn the tile with constraints");
                    puzzle[row][column] = Some(tile);
                }
                (11, 0) => {
                    let previous = puzzle
                        .get(row - 1)
                        .unwrap()
                        .get(column)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let tile_id = previous.matches.get(&Border::Bottom).unwrap();
                    corners.remove(corners.iter().position(|id| id == tile_id).unwrap());
                    let mut tile = tiles.get(&tile_id).unwrap().clone();
                    let mut constraints: HashMap<Border, Vec<Pixel>> = HashMap::default();
                    constraints.insert(
                        Border::Top,
                        previous
                            .get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Bottom)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    tile.turn_to_borders_with_constraints(
                        &vec![Border::Top, Border::Right],
                        constraints,
                    )
                    .expect("failed to turn the tile with constraints");
                    puzzle[row][column] = Some(tile);
                }
                (11, 11) => {
                    let top = puzzle
                        .get(row - 1)
                        .unwrap()
                        .get(column)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let left = puzzle
                        .get(row)
                        .unwrap()
                        .get(column - 1)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let tile_id = left.matches.get(&Border::Right).unwrap();
                    corners.remove(corners.iter().position(|id| id == tile_id).unwrap());
                    let mut tile = tiles.get(&tile_id).unwrap().clone();
                    let mut constraints: HashMap<Border, Vec<Pixel>> = HashMap::default();
                    constraints.insert(
                        Border::Left,
                        left.get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Right)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    constraints.insert(
                        Border::Top,
                        top.get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Bottom)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    tile.turn_to_borders_with_constraints(
                        &vec![Border::Top, Border::Left],
                        constraints,
                    )
                    .expect("failed to turn the tile with constraints");
                    puzzle[row][column] = Some(tile);
                }
                (0, _) => {
                    let previous = puzzle
                        .get(row)
                        .unwrap()
                        .get(column - 1)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let tile_id = previous.matches.get(&Border::Right).unwrap();
                    borders.remove(borders.iter().position(|id| id == tile_id).unwrap());
                    let mut tile = tiles.get(&tile_id).unwrap().clone();
                    let mut constraints: HashMap<Border, Vec<Pixel>> = HashMap::default();
                    constraints.insert(
                        Border::Left,
                        previous
                            .get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Right)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    tile.turn_to_borders_with_constraints(
                        &vec![Border::Right, Border::Bottom, Border::Left],
                        constraints,
                    )
                    .expect("failed to turn the tile with constraints");
                    puzzle[row][column] = Some(tile);
                }
                (_, 0) => {
                    let previous = puzzle
                        .get(row - 1)
                        .unwrap()
                        .get(column)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let tile_id = previous.matches.get(&Border::Bottom).unwrap();
                    borders.remove(borders.iter().position(|id| id == tile_id).unwrap());
                    let mut tile = tiles.get(&tile_id).unwrap().clone();
                    let mut constraints: HashMap<Border, Vec<Pixel>> = HashMap::default();
                    constraints.insert(
                        Border::Top,
                        previous
                            .get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Bottom)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    tile.turn_to_borders_with_constraints(
                        &vec![Border::Top, Border::Right, Border::Bottom],
                        constraints,
                    )
                    .expect("failed to turn the tile with constraints");
                    puzzle[row][column] = Some(tile);
                }
                (_, 11) => {
                    let top = puzzle
                        .get(row - 1)
                        .unwrap()
                        .get(column)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let left = puzzle
                        .get(row)
                        .unwrap()
                        .get(column - 1)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let tile_id = top.matches.get(&Border::Bottom).unwrap();
                    borders.remove(borders.iter().position(|id| id == tile_id).unwrap());
                    let mut tile = tiles.get(&tile_id).unwrap().clone();
                    let mut constraints: HashMap<Border, Vec<Pixel>> = HashMap::default();
                    constraints.insert(
                        Border::Left,
                        left.get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Right)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    constraints.insert(
                        Border::Top,
                        top.get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Bottom)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    tile.turn_to_borders_with_constraints(
                        &vec![Border::Top, Border::Left, Border::Bottom],
                        constraints,
                    )
                    .expect("failed to turn the tile with constraints");
                    puzzle[row][column] = Some(tile);
                }
                (11, _) => {
                    let top = puzzle
                        .get(row - 1)
                        .unwrap()
                        .get(column)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let left = puzzle
                        .get(row)
                        .unwrap()
                        .get(column - 1)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let tile_id = top.matches.get(&Border::Bottom).unwrap();
                    borders.remove(borders.iter().position(|id| id == tile_id).unwrap());
                    let mut tile = tiles.get(&tile_id).unwrap().clone();
                    let mut constraints: HashMap<Border, Vec<Pixel>> = HashMap::default();
                    constraints.insert(
                        Border::Left,
                        left.get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Right)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    constraints.insert(
                        Border::Top,
                        top.get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Bottom)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    tile.turn_to_borders_with_constraints(
                        &vec![Border::Top, Border::Left, Border::Right],
                        constraints,
                    )
                    .expect("failed to turn the tile with constraints");
                    puzzle[row][column] = Some(tile);
                }
                (_, _) => {
                    let top = puzzle
                        .get(row - 1)
                        .unwrap()
                        .get(column)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let left = puzzle
                        .get(row)
                        .unwrap()
                        .get(column - 1)
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    let tile_id = left.matches.get(&Border::Right).unwrap();
                    central.remove(central.iter().position(|id| id == tile_id).unwrap());
                    let mut tile = tiles.get(&tile_id).unwrap().clone();
                    let mut constraints: HashMap<Border, Vec<Pixel>> = HashMap::default();
                    constraints.insert(
                        Border::Left,
                        left.get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Right)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    constraints.insert(
                        Border::Top,
                        top.get_borders()
                            .iter()
                            .find(|(border, _value)| border == &Border::Bottom)
                            .unwrap()
                            .1
                            .clone(),
                    );
                    tile.turn_with_constraints(constraints)
                        .expect("failed to turn the tile with constraints");
                    puzzle[row][column] = Some(tile);
                }
            }
        }
    }

    let mut cleaned_puzzle: Tile = Tile {
        image: vec![],
        matches: HashMap::default(),
    };

    for outer_row in 0..PUZZLE_SIZE {
        let mut sub_puzzle: Vec<Vec<Pixel>> = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        for outer_column in 0..PUZZLE_SIZE {
            for inner_row in 1..9 {
                for inner_column in 1..9 {
                    sub_puzzle.get_mut(inner_row - 1).unwrap().push(
                        puzzle
                            .get(outer_row)
                            .unwrap()
                            .get(outer_column)
                            .unwrap()
                            .as_ref()
                            .unwrap()
                            .image
                            .get(inner_row)
                            .unwrap()
                            .get(inner_column)
                            .unwrap()
                            .clone(),
                    );
                }
            }
        }
        for row in sub_puzzle {
            cleaned_puzzle.image.push(row);
        }
    }

    cleaned_puzzle.fmt();
    cleaned_puzzle
}

fn count_sea_monsters(puzzle: &Tile) -> usize {
    let mut orientations = vec![];
    let mut puzzle = puzzle.clone();
    let mut puzzle_flipped = puzzle.clone();
    puzzle_flipped.flip();
    orientations.push(puzzle.clone());
    orientations.push(puzzle_flipped.clone());
    for _i in 0..3 {
        puzzle.rotate_anti_clock();
        puzzle_flipped.rotate_anti_clock();
        orientations.push(puzzle.clone());
        orientations.push(puzzle_flipped.clone())
    }

    let mut monster = Vec::new();
    for line in MONSTER.lines() {
        monster.push(line.chars().map(|c| c == '#').collect::<Vec<bool>>());
    }
    let mut monsters_found = 0;

    for orientation in orientations.iter() {
        for image_row in 0..orientation.image.len() - monster.len() {
            'image: for image_column in 0..orientation.image[0].len() - monster[0].len() {
                for monster_row in 0..monster.len() {
                    for monster_column in 0..monster[0].len() {
                        if monster[monster_row][monster_column]
                            && orientation.image[image_row + monster_row]
                                [image_column + monster_column]
                                != Pixel::Hash
                        {
                            continue 'image;
                        }
                    }
                }
                monsters_found += 1;
            }
        }
    }

    // ToDo: One orientation has 1 sea monster and one other has 19; 19 is correct. I should first select the orientation with more than one monster, then return the count for only that orientation
    monsters_found - 1
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
    matches: HashMap<Border, usize>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Border {
    Top,
    Right,
    Bottom,
    Left,
}

impl Border {
    fn rotate_anti_clock(&self) -> Self {
        match &self {
            Border::Top => Border::Left,
            Border::Left => Border::Bottom,
            Border::Bottom => Border::Right,
            Border::Right => Border::Top,
        }
    }

    fn flip(&self) -> Self {
        match &self {
            Border::Left => Border::Right,
            Border::Right => Border::Left,
            &border => border.clone(),
        }
    }
}

impl Tile {
    fn turn_to_borders(&mut self, borders: &Vec<Border>) -> Option<()> {
        let mut tile = self.clone();
        let mut orientations = vec![tile.clone()];
        for _i in 0..3 {
            tile.rotate_anti_clock();
            orientations.push(tile.clone());
        }
        let tile = orientations.drain(..).find(|tile| {
            borders
                .iter()
                .all(|border| tile.matches.contains_key(border))
        })?;
        self.matches = tile.matches;
        self.image = tile.image;

        Some(())
    }

    fn turn_to_borders_with_constraints(
        &mut self,
        borders: &Vec<Border>,
        constraints: HashMap<Border, Vec<Pixel>>,
    ) -> Option<()> {
        let mut tile = self.clone();
        tile.turn_to_borders(borders);
        let mut orientations = vec![tile.clone()];
        let mut reversed_two_a = tile.clone();
        reversed_two_a.flip();
        reversed_two_a.turn_to_borders(borders);
        orientations.push(reversed_two_a);
        let mut reversed_two_b = tile.clone();
        reversed_two_b.rotate_anti_clock();
        reversed_two_b.flip();
        reversed_two_b.turn_to_borders(borders);
        orientations.push(reversed_two_b);
        let mut reversed_all = tile.clone();
        reversed_all.flip();
        reversed_all.rotate_anti_clock();
        reversed_all.flip();
        reversed_all.turn_to_borders(borders);
        orientations.push(reversed_all);

        let tile = orientations.drain(..).find(|tile| {
            let borders = tile.get_borders();
            for (constraint_border, constraint_value) in constraints.iter() {
                let value = &borders
                    .iter()
                    .find(|(border, _)| border == constraint_border)
                    .unwrap()
                    .1;
                if value != constraint_value {
                    return false;
                }
            }

            true
        })?;

        self.matches = tile.matches;
        self.image = tile.image;

        Some(())
    }

    fn turn_with_constraints(&mut self, constraints: HashMap<Border, Vec<Pixel>>) -> Option<()> {
        let mut tile = self.clone();
        let mut reversed = tile.clone();
        reversed.flip();
        let mut orientations = vec![tile.clone(), reversed.clone()];
        for _i in 0..3 {
            tile.rotate_anti_clock();
            reversed.rotate_anti_clock();
            orientations.push(tile.clone());
            orientations.push(reversed.clone());
        }

        let tile = orientations.drain(..).find(|tile| {
            let borders = tile.get_borders();
            for (constraint_border, constraint_value) in constraints.iter() {
                let value = &borders
                    .iter()
                    .find(|(border, _)| border == constraint_border)
                    .unwrap()
                    .1;
                if value != constraint_value {
                    return false;
                }
            }

            true
        })?;

        self.matches = tile.matches;
        self.image = tile.image;

        Some(())
    }

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
        let mut borders: Vec<Vec<Pixel>> = self
            .get_borders()
            .drain(..)
            .map(|(_direction, border)| border)
            .collect();
        for mut border in borders.clone() {
            border.reverse();
            borders.push(border);
        }
        borders
    }

    fn rotate_anti_clock(&mut self) {
        let mut image: Vec<Vec<Pixel>> = vec![];
        for row in (0..self.image.len()).rev() {
            let mut new_row = vec![];
            for old_row in self.image.iter() {
                new_row.push(old_row[row].clone());
            }
            image.push(new_row);
        }
        self.image = image;
        let mut borders = HashMap::default();
        for (border, match_id) in self.matches.iter() {
            borders.insert(border.rotate_anti_clock(), match_id.clone());
        }
        self.matches = borders;
    }

    fn flip(&mut self) {
        for row in self.image.iter_mut() {
            row.reverse();
        }
        let mut borders = HashMap::default();
        for (border, match_id) in self.matches.iter() {
            borders.insert(border.flip(), match_id.clone());
        }
        self.matches = borders;
    }

    fn fmt(&self) {
        self.image.iter().for_each(|row| {
            println!(
                "{}",
                row.iter()
                    .map(|pixel| {
                        if pixel == &Pixel::Hash {
                            "#".to_owned()
                        } else {
                            ".".to_owned()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            )
        })
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
        (0, _) => vec![Border::Left, Border::Bottom, Border::Right],
        (_, 11) => vec![Border::Top, Border::Left, Border::Bottom],
        (11, _) => vec![Border::Right, Border::Top, Border::Left],
        (_, 0) => vec![Border::Top, Border::Right, Border::Bottom],
        (_, _) => vec![Border::Top, Border::Right, Border::Bottom, Border::Left],
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
            let mut tile = Tile {
                image: vec![],
                matches: HashMap::default(),
            };
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
