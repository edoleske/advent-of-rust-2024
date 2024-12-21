use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(21);

const KEYPAD: &str = "789456123X0A";
const DPAD: &str = "X^A<v>";

type Cache = HashMap<(Index, Index, u64), u64>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Index(u32, u32);

struct Node {
    position: Index,
    presses: String,
}

impl Node {
    fn new(position: Index, presses: String) -> Self {
        Self { position, presses }
    }

    fn up(&self) -> Self {
        Self::new(Index(self.position.0, self.position.1 - 1), format!("{}^", self.presses))
    }

    fn down(&self) -> Self {
        Self::new(Index(self.position.0, self.position.1 + 1), format!("{}v", self.presses))
    }

    fn left(&self) -> Self {
        Self::new(Index(self.position.0 - 1, self.position.1), format!("{}<", self.presses))
    }

    fn right(&self) -> Self {
        Self::new(Index(self.position.0 + 1, self.position.1), format!("{}>", self.presses))
    }
}

fn dir_pad(from: &Index, to: &Index, n: u64, cache: &mut Cache) -> u64 {
    if let Some(prev) = cache.get(&(to.clone(), from.clone(), n)) {
        return *prev;
    }

    let mut result = u64::MAX;

    let mut queue = VecDeque::new();
    queue.push_back(Node::new(from.clone(), "".to_string()));

    while let Some(node) = queue.pop_front() {
        if node.position == *to {
            let next_result = robot(format!("{}A", node.presses), n - 1, cache);
            if next_result < result {
                result = next_result;
            }
            continue;
        }
        if node.position.0 == 0 && node.position.1 == 0 {
            continue;
        } else {
            if node.position.0 < to.0 {
                queue.push_back(node.right());
            } else if node.position.0 > to.0 {
                queue.push_back(node.left());
            }
            if node.position.1 < to.1 {
                queue.push_back(node.down());
            } else if node.position.1 > to.1 {
                queue.push_back(node.up());
            }
        }
    }

    cache.insert((to.clone(), from.clone(), n), result);
    result
}

fn robot(presses: String, robots: u64, cache: &mut Cache) -> u64 {
    if robots <= 1 {
        return presses.len() as u64;
    }

    let mut result = 0;
    let mut current = Index(2, 0);

    for c in presses.chars() {
        for y in 0..2 {
            for x in 0..3 {
                if DPAD.chars().nth(y * 3 + x) == Some(c) {
                    let to = Index(x as u32, y as u32);
                    result += dir_pad(&current, &to, robots, cache);
                    current = to;
                }
            }
        }
    }

    result
}

fn get_shortest(from: &Index, to: &Index, n: u64, cache: &mut Cache) -> u64 {
    let mut result = u64::MAX;

    let mut queue = VecDeque::new();
    queue.push_back(Node::new(*from, "".to_string()));

    while let Some(node) = queue.pop_front() {
        if node.position == *to {
            let next_result = robot(format!("{}A", node.presses), n, cache);
            if next_result < result {
                result = next_result;
            }
            continue;
        }
        if node.position.0 == 0 && node.position.1 == 3 {
            continue;
        } else {
            if node.position.0 < to.0 {
                queue.push_back(node.right());
            } else if node.position.0 > to.0 {
                queue.push_back(node.left());
            }
            if node.position.1 < to.1 {
                queue.push_back(node.down());
            } else if node.position.1 > to.1 {
                queue.push_back(node.up());
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;

    let mut cache: Cache = HashMap::new();

    for line in input.lines() {
        let mut result = 0;

        let mut current = Index(2, 3);
        for c in line.trim().chars() {
            for y in 0..4 {
                for x in 0..3 {
                    if KEYPAD.chars().nth(y * 3 + x) == Some(c) {
                        let to = Index(x as u32, y as u32);
                        result += get_shortest(&current, &to, 3, &mut cache);
                        current = to;
                    }
                }
            }
        }

        let code_num = line.chars()
            .map_while(|c| c.to_digit(10))
            .fold(0, |acc, digit| acc * 10 + digit);
        sum += result * code_num as u64;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;

    let mut cache: Cache = HashMap::new();

    for line in input.lines() {
        let mut result = 0;

        let mut current = Index(2, 3);
        for c in line.trim().chars() {
            for y in 0..4 {
                for x in 0..3 {
                    if KEYPAD.chars().nth(y * 3 + x) == Some(c) {
                        let to = Index(x as u32, y as u32);
                        result += get_shortest(&current, &to, 26, &mut cache);
                        current = to;
                    }
                }
            }
        }

        let code_num = line.chars()
            .map_while(|c| c.to_digit(10))
            .fold(0, |acc, digit| acc * 10 + digit);
        sum += result * code_num as u64;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
