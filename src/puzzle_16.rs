use std::collections::HashMap;

fn ticket_scanning_error_rate(input: Vec<String>) -> usize {
    let (rules, _own_ticket, tickets): (Vec<TicketRule>, Ticket, Vec<Ticket>) = parse_input(input);

    tickets.iter().fold(0, |acc, ticket| {
        if let Some(count) = ticket.sum_up_invalid_values(&rules) {
            acc + count
        } else {
            acc
        }
    })
}

fn multiply_departure_fields(input: Vec<String>) -> usize {
    let (rules, own_ticket, mut tickets): (Vec<TicketRule>, Ticket, Vec<Ticket>) =
        parse_input(input);

    let valid_tickets: Vec<Ticket> = tickets
        .drain(..)
        .filter(|ticket| ticket.sum_up_invalid_values(&rules).is_none())
        .collect();
    let mut possible_rule_positions: HashMap<String, Vec<usize>> = HashMap::default();

    for rule in rules {
        let positions = own_ticket
            .fields
            .clone()
            .iter()
            .enumerate()
            .filter(|(index, _own_value)| {
                for ticket in &valid_tickets {
                    if !rule.is_valid_value(ticket.fields.get(*index).unwrap().clone()) {
                        return false;
                    }
                }
                true
            })
            .map(|(index, _own_value)| index)
            .collect();

        possible_rule_positions.insert(rule.field_name, positions);
    }

    let mut positions: HashMap<String, usize> = HashMap::default();
    loop {
        if possible_rule_positions.is_empty() {
            break;
        }
        for (name, options) in &possible_rule_positions {
            if options.len() == 1 {
                positions.insert(name.to_owned(), options.first().unwrap().clone());
            }
        }
        for (name, value) in &positions {
            possible_rule_positions.remove(name);
            possible_rule_positions = possible_rule_positions
                .drain()
                .map(|(name, mut entries)| {
                    (
                        name,
                        entries
                            .drain(..)
                            .filter(|option| option != value)
                            .collect::<Vec<usize>>(),
                    )
                })
                .collect();
        }
    }
    positions.drain().fold(1, |acc, (name, position)| {
        if name.starts_with("departure") {
            return acc * own_ticket.fields.get(position).unwrap();
        }
        acc
    })
}

fn parse_input(input: Vec<String>) -> (Vec<TicketRule>, Ticket, Vec<Ticket>) {
    let mut lines = input.iter();
    let mut line = lines.next();

    let mut rules = vec![];
    while line.is_some() && line.unwrap().contains(": ") {
        let mut parts = line.unwrap().split(": ");
        let mut rule = TicketRule {
            field_name: parts.next().unwrap().to_owned(),
            allowed_value_ranges: vec![],
        };

        let mut ranges = parts.next().unwrap().split(" or ");
        let mut range = ranges.next();
        while range.is_some() {
            let mut bounds = range.unwrap().split("-");
            let lower: usize = bounds.next().unwrap().parse().unwrap();
            let upper: usize = bounds.next().unwrap().parse().unwrap();
            rule.allowed_value_ranges.push([lower, upper]);
            range = ranges.next();
        }

        rules.push(rule);
        line = lines.next();
    }
    // own ticket title
    lines.next();
    // own ticket
    line = lines.next();
    let own_ticket = Ticket {
        fields: line
            .unwrap()
            .split(",")
            .map(|field| field.parse::<usize>().unwrap())
            .collect(),
    };

    // empty line
    lines.next();

    // nearby tickets title
    lines.next();
    line = lines.next();
    let mut nearby_tickets = vec![];
    while line.is_some() && line.unwrap().contains(",") {
        let mut ticket = Ticket { fields: vec![] };
        ticket.fields = line
            .unwrap()
            .split(",")
            .map(|field| field.parse::<usize>().unwrap())
            .collect();
        nearby_tickets.push(ticket);
        line = lines.next();
    }

    (rules, own_ticket, nearby_tickets)
}

#[derive(Debug)]
struct TicketRule {
    field_name: String,
    allowed_value_ranges: Vec<[usize; 2]>,
}

impl TicketRule {
    fn is_valid_value(&self, value: usize) -> bool {
        for allowed_value_range in &self.allowed_value_ranges {
            if value >= allowed_value_range[0] && value <= allowed_value_range[1] {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<usize>,
}

impl Ticket {
    fn sum_up_invalid_values(&self, rules: &Vec<TicketRule>) -> Option<usize> {
        let mut error_rate = None;
        for &field in &self.fields {
            let mut valid = false;
            for rule in rules {
                if rule.is_valid_value(field) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                if let Some(current) = error_rate {
                    error_rate = Some(current + field);
                }
                error_rate = Some(field);
            }
        }

        error_rate
    }
}

#[cfg(test)]
mod solve {
    use crate::puzzle_16::{multiply_departure_fields, ticket_scanning_error_rate};
    use crate::utils::read_file_to_vec;

    #[test]
    fn day_16_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_16.txt");
        println!(
            "Ticket scanning error rate: {}",
            ticket_scanning_error_rate(input)
        );
    }

    #[test]
    fn day_16_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_16.txt");
        println!(
            "Multiplied values of the departure fields on own ticket: {}",
            multiply_departure_fields(input)
        );
    }
}
