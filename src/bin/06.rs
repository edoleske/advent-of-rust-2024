use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn advance(&self, position: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
        }
    }
}

struct Map {
    obstacles: Vec<(i32, i32)>,
    start_position: (i32, i32),
    width: usize,
    height: usize,
}

impl Map {
    fn max(&self) -> u32 {
        self.width as u32 * self.height as u32 - self.obstacles.len() as u32
    }
}

fn parse_input(input: &str) -> Map {
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

    Map {
        obstacles,
        start_position,
        width,
        height,
    }
}

fn count_visited(map: &Map) -> u32 {
    let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();
    let mut iter = 0;

    let mut direction = Direction::Up;
    let mut position = map.start_position;
    while position.0 >= 0
        && position.0 < map.width as i32
        && position.1 >= 0
        && position.1 < map.height as i32
    {
        visited.insert((position.0, position.1, direction));
        let mut next_position = direction.advance(position);

        for _ in 0..4 {
            if map.obstacles.contains(&next_position) {
                direction = direction.next();
                next_position = direction.advance(position);
            } else {
                break;
            }
        }

        // Infinite loop detected
        if visited.contains(&(next_position.0, next_position.1, direction)) {
            return map.max();
        }

        // Max iterations as fallback
        iter += 1;
        if iter > map.max() {
            return map.max();
        }

        position = next_position;
    }

    visited
        .iter()
        .map(|v| (v.0, v.1))
        .collect::<HashSet<_>>()
        .len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);

    Some(count_visited(&map))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let mut map = parse_input(input);
    let obstacles = map.obstacles.clone();

    for j in 0..map.height {
        for i in 0..map.width {
            let position = (i as i32, j as i32);
            if map.obstacles.contains(&position) && map.start_position == position {
                continue;
            }

            map.obstacles.push(position);
            let count = count_visited(&map);
            if count == map.max() {
                result += 1;
            }
            map.obstacles = obstacles.clone();
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
