advent_of_code::solution!(3);

fn extract_instruction(string: &str, start: usize) -> Option<&str> {
    let mut pos = start + 4;
    while pos < string.len() {
        match string.chars().nth(pos).unwrap() {
            '0'..='9' | ',' => {}
            ')' => break,
            _ => return None,
        }
        pos += 1;
    }

    let result = &string[start..=pos];
    if !result.contains(',') {
        return None;
    }
    Some(result)
}

fn parse_instruction(instruction: &str) -> u32 {
    let parenthesis = instruction.find('(').unwrap();
    let nums: Vec<u32> = instruction[parenthesis + 1..instruction.len() - 1]
        .split(',')
        .map(|n| n.parse::<u32>())
        .filter(|n| n.is_ok())
        .map(|n| n.unwrap())
        .collect();

    if nums.len() < 2 {
        panic!("Invalid instruction: {}", instruction);
    }

    nums[0] * nums[1]
}

fn check_enabled(indices: &Vec<(usize, bool)>, position: &usize) -> bool {
    let mut last = true;
    for (i, enabled) in indices {
        if i < position {
            last = *enabled;
        } else {
            break;
        }
    }
    last
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut instructions: Vec<&str> = Vec::new();
    let mut seek_str = input;
    while let Some(mul) = seek_str.find("mul(") {
        // Check if invalid characters before next closing parenthesis
        match extract_instruction(seek_str, mul) {
            Some(instruction) => instructions.push(instruction),
            None => {}
        }

        seek_str = &seek_str[mul + 4..];
    }

    let mut result = 0;
    for instruction in instructions {
        result += parse_instruction(instruction);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut instructions: Vec<&str> = Vec::new();

    let mut enabled_indices: Vec<(usize, bool)> = input
        .match_indices("do()")
        .map(|pos| (pos.0, true))
        .chain(
            input
                .match_indices("don't()")
                .map(|pos| (pos.0, false))
                .collect::<Vec<(usize, bool)>>(),
        )
        .collect();
    enabled_indices.sort_by(|a, b| a.0.cmp(&b.0));

    let mut seek_pos: usize = 0;
    while let Some(mul) = input[seek_pos..].find("mul(") {
        // Check if invalid characters before next closing parenthesis
        match extract_instruction(&input[seek_pos..], mul) {
            Some(instruction) => {
                if check_enabled(&enabled_indices, &(mul + seek_pos)) {
                    instructions.push(instruction)
                }
            }
            None => {}
        }

        seek_pos = seek_pos + mul + 4;
    }

    let mut result = 0;
    for instruction in instructions {
        result += parse_instruction(instruction);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
