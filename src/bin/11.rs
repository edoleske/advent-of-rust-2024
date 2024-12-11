use std::collections::HashMap;

advent_of_code::solution!(11);

struct Arrangement {
    stones: HashMap<u64, u64>,
}

impl Arrangement {
    fn new(input: &str) -> Arrangement {
        let mut stones: HashMap<u64, u64> = HashMap::new();
        let starter_stones: Vec<u64> = input
            .trim()
            .split_whitespace()
            .map(|token| token.parse::<u64>().unwrap())
            .collect();

        for stone in starter_stones {
            *stones.entry(stone).or_insert(0) += 1;
        }

        Arrangement { stones }
    }

    fn blink(&mut self) {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();

        for (&n, &count) in &self.stones {
            if n == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if (n.ilog10() + 1) % 2 == 0 {
                let l = n.ilog10() + 1;
                let factor = 10u64.pow(l / 2);
                let half1 = n / factor;
                let half2 = n - half1 * factor;

                *new_stones.entry(half1).or_insert(0) += count;
                *new_stones.entry(half2).or_insert(0) += count;
            } else {
                *new_stones.entry(n * 2024).or_insert(0) += count;
            }
        }

        self.stones = new_stones;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut arrangement = Arrangement::new(input);

    for _ in 0..25 {
        arrangement.blink();
    }

    Some(arrangement.stones.values().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut arrangement = Arrangement::new(input);

    for _ in 0..75 {
        arrangement.blink();
    }

    Some(arrangement.stones.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
