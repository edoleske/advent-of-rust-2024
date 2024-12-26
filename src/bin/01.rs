use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .filter_map(|l| l.trim().split_whitespace().collect_tuple())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for (l, r) in parse_input(input) {
        left.push(l.parse::<i32>().unwrap());
        right.push(r.parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut answer: u32 = 0;
    for i in 0..left.len() {
        answer += (left[i] - right[i]).abs() as u32;
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<i32> = Vec::new();
    let mut right_hashmap: HashMap<i32, u32> = HashMap::new();

    for (l, r) in parse_input(input) {
        left.push(l.parse::<i32>().unwrap());
        *right_hashmap.entry(r.parse::<i32>().unwrap()).or_insert(0) += 1;
    }

    let mut answer: u32 = 0;
    for num in left {
        match right_hashmap.get(&num) {
            Some(count) => answer += num as u32 * count,
            None => {}
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
