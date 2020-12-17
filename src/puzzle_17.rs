use crate::utils::{get_neighbor_positions, Position};

fn active_cubes_after_six_cycles_in_3d(input: Vec<String>) -> usize {
    // input 8x8 -> 6 cycles -> let's simulate 22x22 (no edge cases)
    let mut space: [[[bool; 22]; 22]; 22] = [[[false; 22]; 22]; 22];
    let central_layer = 10;
    for (x, line) in input.iter().enumerate() {
        for (y, char) in line.chars().enumerate() {
            match char {
                '#' => space[central_layer][x + 7][y + 7] = true,
                _ => (),
            }
        }
    }

    let mut cycle = 0;

    while cycle < 6 {
        let copy_space = space.clone();
        let mut z = 1;
        while z < 21 {
            let mut x = 1;
            while x < 21 {
                let mut y = 1;
                while y < 21 {
                    let active = count_active_neighbors_3d(&copy_space, &z, &x, &y);
                    if copy_space[z][x][y] && (active < 2 || active > 3) {
                        space[z][x][y] = false;
                    } else if !copy_space[z][x][y] && active == 3 {
                        space[z][x][y] = true;
                    }
                    y += 1;
                }
                x += 1;
            }
            z += 1;
        }

        cycle += 1;
        println!(
            "Completed cycle {}. Active: {}",
            cycle,
            count_active_cells_3d(&space)
        );
    }

    count_active_cells_3d(&space)
}

fn count_active_cells_3d(space: &[[[bool; 22]; 22]; 22]) -> usize {
    let mut count = 0;
    let mut z = 0;
    while z < 22 {
        let mut x = 0;
        while x < 22 {
            let mut y = 0;
            while y < 22 {
                if space[z][x][y] {
                    count += 1;
                }
                y += 1;
            }
            x += 1
        }
        z += 1;
    }

    count
}

fn count_active_neighbors_3d(
    space: &[[[bool; 22]; 22]; 22],
    z: &usize,
    x: &usize,
    y: &usize,
) -> usize {
    let mut neighbors = get_neighbor_positions(
        Position {
            column: x.clone(),
            row: y.clone(),
        },
        22,
        22,
    );
    let diff: usize = if space[*z][*x][*y] { 1 } else { 0 };
    neighbors.push(Position {
        column: x.clone(),
        row: y.clone(),
    });
    neighbors
        .iter()
        .map(|position| {
            let mut count = 0;
            if space[z - 1][position.column][position.row] {
                count += 1;
            }
            if space[z.clone()][position.column][position.row] {
                count += 1;
            }
            if space[z + 1][position.column][position.row] {
                count += 1;
            }
            count
        })
        .fold(0, |acc, count| acc + count)
        - diff
}

fn active_cubes_after_six_cycles_in_4d(input: Vec<String>) -> usize {
    // input 8x8 -> 6 cycles -> let's simulate 22x22 (no edge cases)
    let mut space: [[[[bool; 22]; 22]; 22]; 22] = [[[[false; 22]; 22]; 22]; 22];
    let central_layer_z = 10;
    let central_layer_w = 10;
    for (x, line) in input.iter().enumerate() {
        for (y, char) in line.chars().enumerate() {
            match char {
                '#' => space[central_layer_z][central_layer_w][x + 7][y + 7] = true,
                _ => (),
            }
        }
    }

    let mut cycle = 0;

    while cycle < 6 {
        let copy_space = space.clone();
        let mut w = 1;
        while w < 21 {
            let mut z = 1;
            while z < 21 {
                let mut x = 1;
                while x < 21 {
                    let mut y = 1;
                    while y < 21 {
                        let active = count_active_neighbors_4d(&copy_space, &w, &z, &x, &y);
                        if copy_space[w][z][x][y] && (active < 2 || active > 3) {
                            space[w][z][x][y] = false;
                        } else if !copy_space[w][z][x][y] && active == 3 {
                            space[w][z][x][y] = true;
                        }
                        y += 1;
                    }
                    x += 1;
                }
                z += 1;
            }
            w += 1;
        }

        cycle += 1;
        println!(
            "Completed cycle {}. Active: {}",
            cycle,
            count_active_cells_4d(&space)
        );
    }

    count_active_cells_4d(&space)
}

fn count_active_cells_4d(space: &[[[[bool; 22]; 22]; 22]; 22]) -> usize {
    let mut count = 0;
    let mut w = 0;
    while w < 22 {
        count += count_active_cells_3d(&space[w]);
        w += 1;
    }

    count
}

fn count_active_neighbors_4d(
    space: &[[[[bool; 22]; 22]; 22]; 22],
    w: &usize,
    z: &usize,
    x: &usize,
    y: &usize,
) -> usize {
    let mut neighbors = get_neighbor_positions(
        Position {
            column: x.clone(),
            row: y.clone(),
        },
        22,
        22,
    );
    let diff: usize = if space[*w][*z][*x][*y] { 1 } else { 0 };
    neighbors.push(Position {
        column: x.clone(),
        row: y.clone(),
    });
    neighbors
        .iter()
        .map(|position| {
            let mut count = 0;
            let mut close_z = z - 1;
            while close_z <= z + 1 {
                let mut close_w = w - 1;
                while close_w <= w + 1 {
                    if space[close_w][close_z][position.column][position.row] {
                        count += 1;
                    }
                    close_w += 1;
                }
                close_z += 1;
            }
            count
        })
        .fold(0, |acc, count| acc + count)
        - diff
}

#[cfg(test)]
mod solve {
    use crate::puzzle_17::{
        active_cubes_after_six_cycles_in_3d, active_cubes_after_six_cycles_in_4d,
    };
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_17_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_17.txt");
        println!(
            "Active cubes after 6 cycles in 3d: {}",
            active_cubes_after_six_cycles_in_3d(input)
        );
    }

    #[test]
    fn day_17_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_17.txt");
        println!(
            "Active cubes after 6 cycles in 4d: {}",
            active_cubes_after_six_cycles_in_4d(input)
        );
    }
}
