use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(24);

#[derive(Clone)]
struct Connection {
    a: String,
    b: String,
    operator: String,
    output: String,
}

impl Connection {
    fn new(line: &str) -> Self {
        let tokens = line.trim().split_whitespace().collect::<Vec<_>>();
        if tokens.len() < 5 {
            panic!("Cannot parse line into connection: {}", line);
        }

        Self {
            a: tokens[0].to_string(),
            b: tokens[2].to_string(),
            operator: tokens[1].to_string(),
            output: tokens[4].to_string(),
        }
    }

    fn operate(&self, state: &mut HashMap<String, bool>) {
        let a = state[&self.a];
        let b = state[&self.b];

        match self.operator.as_str() {
            "AND" => {
                state.insert(self.output.clone(), a && b);
            }
            "OR" => {
                state.insert(self.output.clone(), a || b);
            }
            "XOR" => {
                state.insert(self.output.clone(), a ^ b);
            }
            _ => panic!("Unknown operator: {}", self.operator),
        }
    }
}

fn parse_input(input: &str) -> (HashMap<String, bool>, Vec<Connection>) {
    let mut state: HashMap<String, bool> = HashMap::new();
    let mut connections: Vec<Connection> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if line.contains("-") {
            connections.push(Connection::new(&line));
        } else {
            let (label, value) = line.trim().split_once(": ").unwrap();
            state.insert(label.to_string(), value == "1");
        }
    }

    (state, connections)
}

fn parse_number(state: &HashMap<String, bool>, prefix: char) -> u64 {
    let mut result = 0;

    for (k, _) in state.iter().filter(|(k, v)| k.starts_with(prefix) && **v) {
        let bit_index = k[1..].parse::<u32>().unwrap();
        result += 1 << bit_index;
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut state, connections) = parse_input(input);

    let mut queue = VecDeque::from(connections);
    while let Some(connection) = queue.pop_front() {
        if !state.contains_key(&connection.a) || !state.contains_key(&connection.b) {
            queue.push_back(connection);
            continue;
        }
        
        connection.operate(&mut state);
    }

    Some(parse_number(&state, 'z'))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut state, connections) = parse_input(input);

    let target = parse_number(&state, 'x') + parse_number(&state, 'y');

    // This was easiest to work out by hand :)
    // Todo: clean this up and get it working programmatically after Christmas.
    let mut connections_copy = connections.clone();
    for connection in connections_copy.iter_mut() {
        match connection.output.as_str() {
            "z18" => connection.output = "qgd".to_string(),
            "qgd" => connection.output = "z18".to_string(),
            "z10" => connection.output = "mwk".to_string(),
            "mwk" => connection.output = "z10".to_string(),
            "z33" => connection.output = "gqp".to_string(),
            "gqp" => connection.output = "z33".to_string(),
            "hsw" => connection.output = "jmh".to_string(),
            "jmh" => connection.output = "hsw".to_string(),
            _ => {},
        }
    }

    let mut queue = VecDeque::from(connections_copy);
    while let Some(connection) = queue.pop_front() {
        if !state.contains_key(&connection.a) || !state.contains_key(&connection.b) {
            queue.push_back(connection);
            continue;
        }

        connection.operate(&mut state);
    }

    let output = parse_number(&state, 'z');
    println!("{:b}", target);
    println!("{:b}", output);
    if output == target {
        println!("Success!");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
        
        let result2 = part_one(&advent_of_code::template::read_file_part("examples", DAY, 0));
        assert_eq!(result2, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
