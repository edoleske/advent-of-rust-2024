use std::collections::HashSet;

advent_of_code::solution!(6);

enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }

    fn advance(&self, position: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1)
        }
    }
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, (i32, i32), usize, usize) {
    let mut start_position: (i32, i32) = (0, 0);
    let mut obstacles: Vec<(i32, i32)> = Vec::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '^' => start_position = (i as i32, j as i32),
                '#' => obstacles.push((i as i32, j as i32)),
                _ => {}
            }
        }
    }

    (obstacles, start_position, width, height)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let (obstacles, start_position, width, height) = parse_input(input);

    let mut direction = Direction::Up;
    let mut position = start_position;
    while position.0 >= 0 && position.0 < width as i32 && position.1 >= 0 && position.1 < height as i32 {
        visited.insert(position);
        let mut next_position = direction.advance(position);

        for _ in 0..4 {
            if obstacles.contains(&next_position) {
                direction = direction.next();
                next_position = direction.advance(position);
            } else {
                break;
            }
        }

        position = next_position;
    }

    Some(visited.len() as u32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
