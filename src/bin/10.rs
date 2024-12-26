use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

struct Map {
    data: Vec<String>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        Map {
            data: input.lines().map(|l| l.trim().to_string()).collect(),
            width: input.lines().next().unwrap().trim().len(),
            height: input.lines().count(),
        }
    }

    fn get(&self, x: i32, y: i32) -> char {
        self.data[y as usize].chars().nth(x as usize).unwrap()
    }

    fn get_adjacent(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut adjacent = Vec::new();
        let target = std::char::from_u32(self.get(x, y) as u32 + 1).unwrap();

        if x > 0 && self.get(x - 1, y) == target {
            adjacent.push((x - 1, y));
        }
        if y > 0 && self.get(x, y - 1) == target {
            adjacent.push((x, y - 1));
        }
        if x < self.width as i32 - 1 && self.get(x + 1, y) == target {
            adjacent.push((x + 1, y));
        }
        if y < self.height as i32 - 1 && self.get(x, y + 1) == target {
            adjacent.push((x, y + 1));
        }

        adjacent
    }
}

fn score_trailhead(map: &Map, x: i32, y: i32) -> (u32, u32) {
    let mut trail_count: u32 = 0;
    let mut reachable: HashSet<(i32, i32)> = HashSet::new();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((x, y));

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.extend(map.get_adjacent(x, y).into_iter());

    while let Some((xq, yq)) = queue.pop_front() {
        visited.insert((xq, yq));

        for adjacent in map.get_adjacent(xq, yq).into_iter() {
            if visited.contains(&adjacent) {
                continue;
            }

            if map.get(adjacent.0, adjacent.1) == '9' {
                trail_count += 1;
                reachable.insert(adjacent);
            } else {
                queue.push_back(adjacent);
            }
        }
    }

    (trail_count, reachable.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let map = Map::new(input);

    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x as i32, y as i32) == '0' {
                result += score_trailhead(&map, x as i32, y as i32).1;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let map = Map::new(input);

    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x as i32, y as i32) == '0' {
                result += score_trailhead(&map, x as i32, y as i32).0;
            }
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
        assert_eq!(result, Some(36));

        let result0 = part_one(&advent_of_code::template::read_file_part("examples", DAY, 0));
        assert_eq!(result0, Some(1));

        let result1 = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result1, Some(2));

        let result2 = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result2, Some(4));

        let result3 = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result3, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));

        let result0 = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result0, Some(3));

        let result1 = part_two(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result1, Some(13));

        let result2 = part_two(&advent_of_code::template::read_file_part("examples", DAY, 6));
        assert_eq!(result2, Some(227));
    }
}
