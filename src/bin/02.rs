advent_of_code::solution!(2);

fn parse_line(line: &str) -> Vec<i32> {
    line.trim()
        .split_whitespace()
        .filter(|token| !token.trim().is_empty())
        .map(|token| token.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn evaluate_levels(nums: &Vec<i32>) -> bool {
    let mut safe = true;
    let direction = nums[1] - nums[0];
    for i in 0..nums.len()-1 {
        let difference = nums[i+1] - nums[i];
        if difference.abs() < 1 || difference.abs() > 3 || difference * direction < 0 {
            safe = false;
        }
    }
    safe
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count: u32 = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        let nums = parse_line(line);
        if nums.len() < 2 {
            panic!("Insufficient numbers found in line: '{}'", line);
        }
        
        if evaluate_levels(&nums) {
            safe_count += 1;
        }
    }

    Some(safe_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count: u32 = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let nums = parse_line(line);if nums.len() < 2 {
            panic!("Insufficient numbers found in line: '{}'", line);
        }
        
        if evaluate_levels(&nums) {
            safe_count += 1;
        } else {
            for i in 0..nums.len() {
                let mut vec_modified = nums.clone();
                vec_modified.remove(i);
                if evaluate_levels(&vec_modified) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    Some(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
