const SUBJECT_NUMBER: usize = 7;

fn get_encryption_key(input: &[String]) -> usize {
    let card_pub_key: usize = input.first().unwrap().parse().unwrap();
    let door_pub_key: usize = input.last().unwrap().parse().unwrap();

    let mut door_loop_size = 0;
    let mut temporary_pub_key = 1;
    while temporary_pub_key != door_pub_key {
        door_loop_size += 1;
        temporary_pub_key = temporary_pub_key * SUBJECT_NUMBER;
        temporary_pub_key = temporary_pub_key % 20201227;
    }

    let mut encryption_key = 1;
    for _round in 0..door_loop_size {
        encryption_key = encryption_key * card_pub_key;
        encryption_key = encryption_key % 20201227;
    }

    encryption_key
}

#[cfg(test)]
mod solve {
    use crate::puzzle_25::get_encryption_key;
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_25_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_25.txt");
        println!("Encryption key: {}", get_encryption_key(&input));
    }
}
