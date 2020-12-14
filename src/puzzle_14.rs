fn sum_up_memory_entries_masking_value(input: Vec<String>) -> u64 {
    let mut program = Program {
        memory: vec![],
        mask: vec![],
    };
    for line in input {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        match key {
            "mask" => {
                program.new_mask(value);
            }
            mem => {
                let position: usize = mem
                    .strip_prefix("mem[")
                    .unwrap()
                    .strip_suffix("]")
                    .unwrap()
                    .parse()
                    .unwrap();
                let value: u64 = value.parse().unwrap();
                program.apply_mask_to_value_and_add_value(position, value);
            }
        }
    }

    program.sum_up_memory()
}

fn sum_up_memory_entries_masking_addresses(input: Vec<String>) -> u64 {
    let mut program = Program {
        memory: vec![],
        mask: vec![],
    };
    for line in input {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        match key {
            "mask" => {
                program.new_mask(value);
            }
            mem => {
                let address: usize = mem
                    .strip_prefix("mem[")
                    .unwrap()
                    .strip_suffix("]")
                    .unwrap()
                    .parse()
                    .unwrap();
                let value: u64 = value.parse().unwrap();
                program.apply_mask_to_address_and_add_value(address, value);
            }
        }
    }

    program.sum_up_memory()
}

struct Program {
    memory: Vec<(usize, u64)>,
    mask: Vec<Option<u8>>,
}

impl Program {
    fn new_mask(&mut self, mask_line: &str) -> () {
        self.mask.clear();
        for char in mask_line.chars() {
            match char {
                '1' => self.mask.push(Some(1)),
                '0' => self.mask.push(Some(0)),
                'X' => self.mask.push(None),
                _ => (),
            }
        }
    }

    fn apply_mask_to_address_and_add_value(&mut self, position: usize, value: u64) -> () {
        let mut binary = format!("{:b}", position);
        let padding = 36 - binary.len();
        binary = ["0".repeat(padding), binary].concat();
        let mut addresses: Vec<Vec<u8>> = vec![vec![]];
        for (index, entry) in self.mask.iter().enumerate() {
            let mut new_addresses: Vec<Vec<u8>> = vec![];
            let new_parts: Vec<u8> = match entry {
                None => vec![0, 1],
                Some(num) => {
                    if num == &0 {
                        let elem: u8 = binary
                            .chars()
                            .nth(index)
                            .unwrap()
                            .to_string()
                            .parse()
                            .unwrap();
                        vec![elem]
                    } else {
                        vec![1]
                    }
                }
            };
            for address in addresses.drain(..) {
                for part in new_parts.clone() {
                    let mut entry = address.clone();
                    entry.push(part);
                    new_addresses.push(entry);
                }
            }
            addresses = new_addresses;
        }
        for address in addresses {
            let position = binary_to_u64(address) as usize;
            self.memory = self
                .memory
                .drain(..)
                .filter(|(pos, _value)| pos != &position)
                .collect();
            self.memory.push((position, value));
        }
    }

    fn apply_mask_to_value_and_add_value(&mut self, position: usize, value: u64) -> () {
        let mut binary = format!("{:b}", value);
        let padding = 36 - binary.len();
        binary = ["0".repeat(padding), binary].concat();
        let mut masked_value: Vec<u8> = vec![];
        for (position, bit) in binary.chars().enumerate() {
            let mask = self.mask.iter().nth(position).unwrap();
            match mask {
                Some(mask) => masked_value.push(mask.clone()),
                None => match bit {
                    '1' => masked_value.push(1),
                    '0' => masked_value.push(0),
                    _ => (),
                },
            }
        }

        let total = binary_to_u64(masked_value);
        self.memory = self
            .memory
            .drain(..)
            .filter(|(pos, _value)| pos != &position)
            .collect();
        self.memory.push((position, total));
    }

    fn sum_up_memory(&self) -> u64 {
        self.memory.iter().fold(0, |acc, (_adr, value)| acc + value)
    }
}

fn binary_to_u64(value: Vec<u8>) -> u64 {
    let mut total: u64 = 0;
    let mut value = value.clone();
    value.reverse();
    for (position, value) in value.iter().enumerate() {
        match value {
            0 => (),
            1 => total += 2_u64.pow(position as u32),
            _ => (),
        }
    }
    total
}

#[cfg(test)]
mod solve {
    use crate::puzzle_14::{
        sum_up_memory_entries_masking_addresses, sum_up_memory_entries_masking_value,
    };
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_14_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_14.txt");
        println!(
            "All memory entries summed up: {}",
            sum_up_memory_entries_masking_value(input)
        );
    }
    #[test]
    fn day_14_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_14.txt");
        println!(
            "All memory entries summed up: {}",
            sum_up_memory_entries_masking_addresses(input)
        );
    }
}
