use std::collections::HashMap;

fn multiply_corner_image_ids(input: Vec<String>) -> Option<usize> {
    let tiles: TileMap = parse_tiles(input);
    let (corners, _borders, _central) = sort_tiles(tiles);

    if corners.len() == 4 {
        return Some(corners.iter().fold(1, |acc, id| acc * id));
    }

    None
}

struct TileMatch {
    id: usize,
    matched: Vec<Border>,
}

fn sort_tiles(tiles: TileMap) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut corners: Vec<usize> = vec![];
    let mut borders: Vec<usize> = vec![];
    let mut central: Vec<usize> = vec![];
    let mut tile_ids: Vec<&usize> = tiles.keys().collect();
    while !tile_ids.is_empty() {
        let starting_id = tile_ids.pop().unwrap();
        let mut other_tiles = tiles.clone();
        let start = other_tiles.remove(starting_id).unwrap();
        let mut matched_borders = 0;
        'border: for border in start.get_borders() {
            for id in other_tiles.keys().collect::<Vec<&usize>>() {
                if other_tiles
                    .get(id)
                    .unwrap()
                    .get_borders_plus_reversed()
                    .contains(&border)
                {
                    matched_borders += 1;
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

#[derive(Debug)]
enum Border {
    Top,
    Right,
    Bottom,
    Left,
}

type TileMap = HashMap<usize, Tile>;

#[derive(Debug, Clone)]
struct Tile {
    image: Vec<Vec<Pixel>>,
}

impl Tile {
    fn get_borders(&self) -> Vec<Vec<Pixel>> {
        let mut borders: Vec<Vec<Pixel>> = vec![];
        borders.push(self.image.first().unwrap().clone());
        borders.push(self.image.last().unwrap().clone());
        let mut left = vec![];
        let mut right = vec![];

        for row in self.image.iter() {
            left.push(row.first().unwrap().clone());
            right.push(row.last().unwrap().clone());
        }
        borders.push(right);
        borders.push(left);
        borders
    }

    fn get_borders_plus_reversed(&self) -> Vec<Vec<Pixel>> {
        let mut borders = self.get_borders();
        for mut border in borders.clone() {
            border.reverse();
            borders.push(border);
        }
        borders
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Pixel {
    Dot,
    Hash,
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
            let mut tile = Tile { image: vec![] };
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
    use crate::puzzle_20::multiply_corner_image_ids;
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_20_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_20.txt");
        println!(
            "Multiple of the corner image IDs: {}",
            multiply_corner_image_ids(input).unwrap()
        );
    }
}
