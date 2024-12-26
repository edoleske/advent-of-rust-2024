use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

fn next_in_sequence(secret: i64) -> i64 {
    let one = ((secret * 64) ^ secret) % 16777216;
    let two = ((one / 32) ^ one) % 16777216;
    ((two * 2048) ^ two) % 16777216
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;

    for l in input.lines() {
        let secret: i64 = l.trim().parse().unwrap();

        let mut result = secret;
        for _ in 0..2000 {
            result = next_in_sequence(result);
        }
        sum += result as u64;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result_map: HashMap<(i8, i8, i8, i8), u32> = HashMap::new();

    for l in input.lines() {
        let mut found: HashSet<(i8, i8, i8, i8)> = HashSet::new();
        let secret: i64 = l.trim().parse().unwrap();

        let mut delta1: i8;
        let mut delta2: i8 = 10;
        let mut delta3: i8 = 10;
        let mut delta4: i8 = 10;
        let mut last_secret = secret;

        for i in 0..2000 {
            let next_secret = next_in_sequence(last_secret);
            delta1 = delta2;
            delta2 = delta3;
            delta3 = delta4;
            delta4 = (next_secret % 10 - last_secret % 10) as i8;
            last_secret = next_secret;

            if i < 3 {
                continue;
            }

            let sequence = (delta1, delta2, delta3, delta4);
            if found.contains(&sequence) {
                continue;
            }
            found.insert(sequence);

            let price = (next_secret % 10) as u32;
            *result_map.entry(sequence).or_insert(0) += price;
        }
    }

    result_map.values().max().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 0));
        assert_eq!(result, Some(23));
    }
}
