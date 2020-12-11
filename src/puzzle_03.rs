fn multiply_trees_on_slopes(slopes: Vec<(usize, usize)>, map: &Vec<Vec<Encounter>>) -> usize {
    let mut trees_multiplied = 1;
    for slope in slopes {
        trees_multiplied *= count_trees_on_slope(slope.0, slope.1, map);
    }
    trees_multiplied
}

fn count_trees_on_slope(
    column_step_width: usize,
    row_step_width: usize,
    map: &Vec<Vec<Encounter>>,
) -> usize {
    let mut trees = 0;
    let mut column = 0;
    let mut row = 0;
    while row < map.len() {
        if let Encounter::TREE = get_step(column, row, map) {
            trees += 1;
        }
        column += column_step_width;
        row += row_step_width;
    }

    trees
}

#[derive(Debug)]
enum Encounter {
    FREE,
    TREE,
}

impl From<char> for Encounter {
    fn from(char: char) -> Self {
        match char {
            '#' => Encounter::TREE,
            '.' => Encounter::FREE,
            _ => {
                println!(
                    "Failed to convert {} to encounter. Falling back to tree",
                    char
                );
                Encounter::TREE
            }
        }
    }
}

fn get_step(x: usize, y: usize, map: &Vec<Vec<Encounter>>) -> &Encounter {
    let map_width = map.first().unwrap().len();
    map.iter()
        .nth(y)
        .unwrap()
        .iter()
        .nth(x % map_width)
        .unwrap()
}

#[cfg(test)]
mod solve {
    use crate::puzzle_03::{count_trees_on_slope, multiply_trees_on_slopes, Encounter};
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_03_1() {
        let input: Vec<Vec<Encounter>> = read_file_to_vec("input/puzzle_03.txt")
            .iter()
            .map(|row| row.chars().map(|char| Encounter::from(char)).collect())
            .collect();
        println!(
            "There are {} trees in the slope",
            count_trees_on_slope(1, 3, &input)
        );
    }

    #[test]
    fn day_03_2() {
        let input: Vec<Vec<Encounter>> = read_file_to_vec("input/puzzle_03.txt")
            .iter()
            .map(|row| row.chars().map(|char| Encounter::from(char)).collect())
            .collect();
        println!(
            "Multiplying trees on all slopes: {}",
            multiply_trees_on_slopes(vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)], &input)
        );
    }
}
