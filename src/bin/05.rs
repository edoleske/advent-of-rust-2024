use std::collections::HashMap;

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if line.contains('|') {
            let nums = line
                .trim()
                .split('|')
                .map(|token| token.parse().unwrap())
                .collect::<Vec<u32>>();
            if nums.len() != 2 {
                panic!("Invalid rule: '{}'", line);
            }

            rules.entry(nums[0]).or_insert(Vec::new()).push(nums[1]);
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

fn is_update_valid(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    let mut valid = true;

    let l = update.len();
    'outer: for i in 1..l {
        let page = &update[l - i];
        if rules.contains_key(page) {
            let before_pages = &update[0..l - i];
            for p in before_pages {
                if rules[page].contains(p) {
                    valid = false;
                    break 'outer;
                }
            }
        }
    }

    valid
}

fn fix_update(rules: &HashMap<u32, Vec<u32>>, update: &mut Vec<u32>) {
    let l = update.len();
    while !is_update_valid(rules, update) {
        for i in 1..l {
            let page = &update[l - i];
            if rules.contains_key(page) {
                let before_pages = &update[0..l - i];
                for p in before_pages {
                    if rules[page].contains(p) {
                        update.swap(l - i - 1, l - i);
                        break;
                    }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let (rules, updates) = parse_input(input);

    for update in updates {
        if is_update_valid(&rules, &update) {
            total += update[update.len() / 2];
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let (rules, updates) = parse_input(input);

    for mut update in updates {
        if !is_update_valid(&rules, &update) {
            fix_update(&rules, &mut update);
            total += update[update.len() / 2];
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
