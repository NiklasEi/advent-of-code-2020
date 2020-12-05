fn find_boarding_pass_with_highest_id(input: Vec<String>) -> usize {
    let mut seat_ids: Vec<usize> = parse_boarding_passes(&input);
    seat_ids.sort_by(|a, b| a.cmp(b));
    seat_ids.last().unwrap().to_owned()
}

fn find_missing_boarding_pass_id(input: Vec<String>) -> usize {
    let mut seat_ids: Vec<usize> = parse_boarding_passes(&input);
    seat_ids.sort_by(|a, b| a.cmp(b));

    // our seat can't be the first or last and is supposed to be surrounded by taken seats
    // Let's combine all seats to go though all pairs and find a pair where a seat is missing between two taken numbers
    let mut higher_ids = seat_ids.clone();
    higher_ids.remove(0);
    seat_ids.pop();
    seat_ids.reverse();

    let combined_id_pairs: Vec<(usize, usize)> = higher_ids
        .iter()
        .map(|id| (id.to_owned(), seat_ids.pop().unwrap()))
        .collect();

    let surrounding_seats: &(usize, usize) =
        combined_id_pairs.iter().find(|(a, b)| a - b == 2).unwrap();
    (surrounding_seats.0 + surrounding_seats.1) / 2
}

fn parse_boarding_passes(input: &Vec<String>) -> Vec<usize> {
    let mut boarding_pass_ids = vec![];
    for boarding_pass in input {
        boarding_pass_ids.push(calc_boarding_pass_id(boarding_pass.to_owned()));
    }

    boarding_pass_ids
}

fn calc_boarding_pass_id(boarding_pass: String) -> usize {
    let rows = &boarding_pass[..7];
    let columns = &boarding_pass[7..];
    let mut row_numbers: Vec<usize> = (0..128).collect();
    for row in rows.chars() {
        let split_point = row_numbers.first().unwrap() + row_numbers.len() / 2;
        let chunks: (Vec<usize>, Vec<usize>) =
            row_numbers.iter().partition(|num| num < &&split_point);
        row_numbers = match &row.to_string()[..] {
            "F" => chunks.0,
            "B" => chunks.1,
            _ => panic!("Invalid row direction given"),
        };
    }
    let mut column_numbers: Vec<usize> = (0..8).collect();
    for column in columns.chars() {
        let split_point = column_numbers.first().unwrap() + column_numbers.len() / 2;
        let chunks: (Vec<usize>, Vec<usize>) =
            column_numbers.iter().partition(|num| num < &&split_point);
        column_numbers = match &column.to_string()[..] {
            "L" => chunks.0,
            "R" => chunks.1,
            _ => panic!("Invalid column direction given"),
        };
    }

    row_numbers.first().unwrap() * 8 + column_numbers.first().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::puzzle_05::{find_boarding_pass_with_highest_id, find_missing_boarding_pass_id};
    use crate::read_file::read_file_to_vec;

    #[test]
    fn solve_day_05_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_05.txt");

        println!(
            "The highest boarding pass id given is: {}",
            find_boarding_pass_with_highest_id(input)
        );
    }

    #[test]
    fn solve_day_05_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_05.txt");

        println!(
            "Missing boarding pass id: {}",
            find_missing_boarding_pass_id(input)
        );
    }
}
