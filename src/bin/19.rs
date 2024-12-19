use std::collections::HashMap;

advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let patterns: Vec<String> = input
        .lines()
        .nth(0)
        .unwrap()
        .split(", ")
        .map(String::from)
        .collect();
    let designs: Vec<String> = input
        .lines()
        .skip(2)
        .map(|s| s.trim().to_string())
        .collect();

    (patterns, designs)
}

fn composable(string: &String, substrings: &Vec<String>, memo: &mut HashMap<String, bool>) -> bool {
    if string.is_empty() {
        return true;
    }

    if memo.contains_key(string) {
        return memo[string];
    }
    memo.insert(string.clone(), false);

    for substring in substrings {
        if string.starts_with(substring)
            && composable(&string[substring.len()..].to_string(), substrings, memo)
        {
            memo.insert(string.clone(), true);
        }
    }

    memo[string]
}

fn count_arrangements(
    string: &String,
    substrings: &Vec<String>,
    memo: &mut HashMap<String, bool>,
    arrangement_cache: &mut HashMap<String, u64>,
) -> u64 {
    if string.is_empty() {
        return 1;
    }

    if arrangement_cache.contains_key(string) {
        return arrangement_cache[string];
    }
    arrangement_cache.insert(string.clone(), 0);

    for substring in substrings {
        if string.starts_with(substring)
            && composable(&string[substring.len()..].to_string(), substrings, memo)
        {
            let sub_count= count_arrangements(&string[substring.len()..].to_string(), substrings, memo, arrangement_cache);
            *arrangement_cache.entry(string.clone()).or_insert(0) += sub_count;
        }
    }

    arrangement_cache[string]
}

pub fn part_one(input: &str) -> Option<u32> {
    let (patterns, designs) = parse_input(input);
    let mut memo: HashMap<String, bool> = HashMap::new();

    let mut result = 0;

    for design in designs {
        if composable(&design, &patterns, &mut memo) {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, designs) = parse_input(input);
    let mut memo: HashMap<String, bool> = HashMap::new();
    let mut arrangement_cache: HashMap<String, u64> = HashMap::new();

    let mut result = 0;

    for design in designs {
        if composable(&design, &patterns, &mut memo) {
            result += count_arrangements(&design, &patterns, &mut memo, &mut arrangement_cache);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
