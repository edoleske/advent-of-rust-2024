use std::collections::{HashSet};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules: HashSet<(u32, u32)> = HashSet::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if line.contains('|') {
            let (s1, s2) = line.trim().split_once('|').unwrap();
            let n1 = s1.parse::<u32>().unwrap();
            let n2 = s2.parse::<u32>().unwrap();
            rules.insert((n1, n2));
        } else {
            let nums = line
                .trim()
                .split(',')
                .map(|token| token.parse().unwrap())
                .collect::<Vec<u32>>();
            updates.push(nums);
        }
    }

    (rules, updates)
}

fn get_sorted_update(rules: &HashSet<(u32, u32)>, update: &Vec<u32>) -> Vec<u32> {
    let mut result = update.clone();

    result.sort_by(|a, b| if rules.contains(&(*a, *b)) {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    });
    
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let (rules, updates) = parse_input(input);

    for update in updates {
        let sorted = get_sorted_update(&rules, &update);
        if update.iter().zip(&sorted).all(|(a, b)| a == b) {
            total += update[update.len() / 2];
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let (rules, updates) = parse_input(input);

    for update in updates {
        let sorted = get_sorted_update(&rules, &update);
        if !update.iter().zip(&sorted).all(|(a, b)| a == b) {
            total += sorted[sorted.len() / 2];
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
