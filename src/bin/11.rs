use std::collections::LinkedList;

advent_of_code::solution!(11);

struct Arrangement {
    stones: Vec<u64>,
}

impl Arrangement {
    fn new(input: &str) -> Arrangement {
        Arrangement {
            stones: input
                .trim()
                .split_whitespace()
                .map(|token| token.parse::<u64>().unwrap())
                .collect(),
        }
    }

    fn blink(&mut self) {
        let mut new_stones: Vec<u64> = Vec::with_capacity(self.stones.len());


        for &n in &self.stones {
            if n == 0 {
                new_stones.push(1);
            } else if (n.ilog10() + 1) % 2 == 0 {
                let l = n.ilog10() + 1;
                let factor = 10u64.pow(l / 2);
                let half1 = n / factor;
                let half2 = n - half1 * factor;

                new_stones.push(half1);
                new_stones.push(half2);
            } else {
                new_stones.push(n * 2024);
            }
        }

        self.stones = new_stones;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut arrangement = Arrangement::new(input);

    for _ in 0..25 {
        arrangement.blink();
    }

    Some(arrangement.stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut arrangement = Arrangement::new(input);

    for i in 0..75 {
        arrangement.blink();
        println!("blink {}", i);
    }

    Some(arrangement.stones.len() as u32)
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
        assert_eq!(result, None);
    }
}
