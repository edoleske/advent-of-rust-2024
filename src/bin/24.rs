use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(24);

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

fn parse_output(state: &HashMap<String, bool>) -> u64 {
    let mut result = 0;

    for (k, _) in state.iter().filter(|(k, v)| k.starts_with('z') && **v) {
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

    Some(parse_output(&state))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
