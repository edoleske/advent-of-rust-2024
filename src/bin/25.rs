advent_of_code::solution!(25);

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Schematic {
    pins: Vec<u8>,
    is_key: bool,
}

impl Schematic {
    fn new(lines: &Vec<&str>) -> Self {
        if lines.len() != 7 {
            panic!("Unexpected input len for schematic: {}", lines.len());
        }

        let l = lines[0].len();
        let mut pins = vec![0u8; l];
        let is_key = lines[0] == "#####";

        for line in lines {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    pins[i] += 1;
                }
            }
        }
        pins = pins.iter().map(|n| n - 1).collect();

        Self { pins, is_key }
    }

    fn fits(&self, other: &Schematic) -> bool {
        if self.is_key == other.is_key {
            return false;
        }

        let mut result = true;

        for i in 0..self.pins.len() {
            if self.pins[i] + other.pins[i] > 5 {
                result = false;
                break;
            }
        }

        result
    }
}

fn parse_schematics(input: &str) -> (Vec<Schematic>, Vec<Schematic>) {
    let mut schematics: Vec<Schematic> = Vec::new();

    let mut lines = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            schematics.push(Schematic::new(&lines));
            lines = Vec::new();
            continue;
        }

        lines.push(line.trim());
    }

    if lines.len() > 0 {
        schematics.push(Schematic::new(&lines));
    }

    (
        schematics.iter().filter(|s| s.is_key).cloned().collect(),
        schematics.iter().filter(|s| !s.is_key).cloned().collect(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (keys, locks) = parse_schematics(input);

    let mut result = 0;

    for key in &keys {
        for lock in &locks {
            if key.fits(&lock) {
                result += 1;
            }
        }
    }

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
