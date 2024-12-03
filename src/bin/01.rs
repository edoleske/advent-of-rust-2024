use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str, left: &mut Vec<i32>, right: &mut Vec<i32>) {
    let lines: Vec<&str> = input
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .collect();
    
    for line in lines {
        let tokens: Vec<&str> = line
            .trim()
            .split(' ')
            .filter(|token| !token.trim().is_empty())
            .collect();

        if tokens.len() != 2 {
            panic!("Error parsing tokens from input: {}", line);
        }

        left.push(tokens[0].parse().unwrap());
        right.push(tokens[1].parse().unwrap());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    parse_input(input, &mut left, &mut right);

    left.sort();
    right.sort();

    let mut answer: u32 = 0;
    let it = left.iter().zip(right.iter());
    for (_, (l, r)) in it.enumerate() {
        answer += (l - r).abs() as u32;
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    parse_input(input, &mut left, &mut right);
    
    let mut right_hashmap: HashMap<i32, u32> = HashMap::new();
    for num in right {
        *right_hashmap.entry(num).or_insert(0) += 1;
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
