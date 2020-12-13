use std::cmp;

fn first_bus_id_times_waiting_time(input: Vec<String>) -> usize {
    let earliest_embark: usize = input.first().unwrap().parse().unwrap();
    let bus_ids: Vec<usize> = input
        .last()
        .unwrap()
        .split(",")
        .map(|id_or_x| id_or_x.parse::<usize>())
        .filter(|parsed| parsed.is_ok())
        .map(|result| result.unwrap())
        .collect();
    let mut closest_departure: (usize, usize) = (0, usize::max_value());
    for id in bus_ids {
        let mut bus: (usize, usize) = (id, 0);
        while bus.1 < earliest_embark {
            bus.1 += id;
        }
        if bus.1 < closest_departure.1 {
            closest_departure = bus;
        }
    }

    (closest_departure.1 - earliest_embark) * closest_departure.0
}

fn first_timestamp_departing_at_offsets(input: Vec<String>) -> usize {
    let mut bus_schedules: Vec<BusSchedule> = vec![];
    for (offset, id) in input.last().unwrap().split(",").enumerate() {
        let parsed = id.parse();
        if parsed.is_err() {
            continue;
        }
        bus_schedules.push(BusSchedule {
            id: parsed.unwrap(),
            offset,
        })
    }
    let mut period = 1;
    let mut timestamp = 0;

    for schedule in bus_schedules {
        let effective_offset = (schedule.id - (schedule.offset % schedule.id)) % schedule.id;

        while timestamp % schedule.id != effective_offset {
            timestamp += period;
        }

        period = lowest_common_multiple(period, schedule.id);
    }
    timestamp
}

struct BusSchedule {
    id: usize,
    offset: usize,
}

impl BusSchedule {
    fn has_correct_position_for_timestamp(&self, timestamp: usize) -> bool {
        let next_timestamp = timestamp + self.id - (timestamp % self.id);
        next_timestamp == timestamp + self.offset
    }
}

fn lowest_common_multiple(first: usize, second: usize) -> usize {
    (first * second) / greatest_common_divisor(first, second)
}

fn greatest_common_divisor(first: usize, second: usize) -> usize {
    let mut max = cmp::max(first, second);
    let mut divisor = cmp::min(first, second);

    loop {
        let left_over = max % divisor;
        if left_over == 0 {
            return divisor;
        }

        max = divisor;
        divisor = left_over;
    }
}

#[cfg(test)]
mod solve {
    use crate::puzzle_13::{first_bus_id_times_waiting_time, first_timestamp_departing_at_offsets};
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_13_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_13.txt");
        println!(
            "First bus ID times waiting time: {}",
            first_bus_id_times_waiting_time(input)
        );
    }

    #[test]
    fn day_13_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_13.txt");
        println!(
            "First timestamp where busses are deporting at offsets: {}",
            first_timestamp_departing_at_offsets(input)
        );
    }
}
