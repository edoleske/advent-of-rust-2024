advent_of_code::solution!(7);

fn parse_line(input: &str) -> (Vec<i64>, i64) {
    let mut numbers: Vec<i64> = Vec::new();
    let mut target: i64 = 0;

    for side in input.trim().split(':') {
        if side.trim().contains(' ') {
            numbers = side
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
        } else {
            target = side.trim().parse().unwrap();
        }
    }

    if numbers.len() < 1 {
        panic!("Not enough numbers: {}", input);
    }

    (numbers, target)
}

fn concat_numbers(x: i64, y: i64) -> Option<i64> {
    match format!("{}{}", x, y).parse::<i64>() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn is_target_possible(target: i64, numbers: &Vec<i64>, concat: bool) -> bool {
    let mut results: Vec<i64> = Vec::new();

    // To start calculate the possible starting results from the first numbers
    results.push(numbers[0] + numbers[1]);
    results.push(numbers[0] * numbers[1]);
    if concat {
        if let Some(c) = concat_numbers(numbers[0], numbers[1]) {
            results.push(c);
        }
    }

    // Calculate two possible results from each previous result and repeat
    for n in numbers[2..].iter() {
        let mut new_results: Vec<i64> = Vec::new();

        for r in &results {
            // The main challenge is that overflows will happen, so used checked_ operators
            // We save time by ignoring results that exceed the target
            if let Some(sum) = r.checked_add(*n) {
                if sum <= target {
                    new_results.push(sum);
                }
            }
            if let Some(product) = r.checked_mul(*n) {
                if product <= target {
                    new_results.push(product);
                }
            }
            if concat {
                if let Some(c) = concat_numbers(*r, *n) {
                    new_results.push(c);
                }
            }
        }

        results = new_results;
    }

    results.contains(&target)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for line in input.lines() {
        let (numbers, target) = parse_line(line);

        if is_target_possible(target, &numbers, false) {
            result += target as u64;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for line in input.lines() {
        let (numbers, target) = parse_line(line);

        if is_target_possible(target, &numbers, true) {
            result += target as u64;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
