fn multiply_one_and_three_joltage_differences(input: &mut Vec<String>) -> usize {
    let mut joltages: Vec<usize> = input
        .drain(..)
        .map(|adapter| adapter.parse::<usize>().unwrap())
        .collect();
    let mut joltage_differences: Vec<usize> = vec![];
    joltages.sort();
    let mut previous_joltage: usize = 0;
    for joltage in joltages.drain(..) {
        joltage_differences.push(joltage - previous_joltage);
        previous_joltage = joltage;
    }
    joltage_differences.push(3);

    let (one, three): (Vec<usize>, Vec<usize>) = joltage_differences
        .drain(..)
        .filter(|&diff| diff == 1 || diff == 3)
        .partition(|&diff| diff == 1);
    one.len() * three.len()
}

fn find_combinations_to_connect_the_adapters(input: &mut Vec<String>) -> usize {
    let mut adapter_joltages: Vec<usize> = input
        .drain(..)
        .map(|adapter| adapter.parse::<usize>().unwrap())
        .collect();
    adapter_joltages.sort();
    let highest_joltage = adapter_joltages.last().unwrap().clone();
    let mut sub_trees: Vec<AdapterSubTree> = find_possible_connections(&0, &adapter_joltages)
        .iter()
        .map(|joltage| AdapterSubTree {
            joltage: joltage.clone(),
            combinations: 1,
        })
        .collect();
    while sub_trees.len() > 1 || sub_trees.first().unwrap().joltage != highest_joltage {
        sub_trees.reverse();
        let lowest_sub_tree = sub_trees.pop().unwrap();
        sub_trees.reverse();
        let joltages_connected_to_lowest_sub_tree =
            find_possible_connections(&lowest_sub_tree.joltage, &adapter_joltages);
        for joltage in joltages_connected_to_lowest_sub_tree {
            let found = sub_trees.iter_mut().find(|tree| tree.joltage == joltage);
            if let Some(tree) = found {
                tree.combinations += lowest_sub_tree.combinations;
            } else {
                sub_trees.push(AdapterSubTree {
                    joltage,
                    combinations: lowest_sub_tree.combinations,
                });
            }
        }
    }
    sub_trees.first().unwrap().combinations
}

struct AdapterSubTree {
    joltage: usize,
    combinations: usize,
}

fn find_possible_connections(last: &usize, input: &Vec<usize>) -> Vec<usize> {
    input
        .clone()
        .drain(..)
        .filter(|connector| connector > last && connector - last < 4)
        .collect()
}

#[cfg(test)]
mod solve {
    use crate::puzzle_10::{
        find_combinations_to_connect_the_adapters, multiply_one_and_three_joltage_differences,
    };
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_10_1() {
        let mut input: Vec<String> = read_file_to_vec("input/puzzle_10.txt");
        println!(
            "Joltage differences 1 and 3 multiplied: {:?}",
            multiply_one_and_three_joltage_differences(&mut input)
        );
    }

    #[test]
    fn day_10_2() {
        let mut input: Vec<String> = read_file_to_vec("input/puzzle_10.txt");
        println!(
            "number of ways to connect the devices: {:?}",
            find_combinations_to_connect_the_adapters(&mut input)
        );
    }
}
