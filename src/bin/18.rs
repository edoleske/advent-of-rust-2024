use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(18);

struct Grid {
    width: i32,
    corrupted: HashSet<(i32, i32)>,
}

impl Grid {
    fn new(coordinates: &Vec<(i32, i32)>) -> Self {
        let width = coordinates
            .iter()
            .map(|&(x, y)| std::cmp::max(x, y))
            .max()
            .unwrap()
            + 1;

        Self {
            width,
            corrupted: HashSet::with_capacity(coordinates.len()),
        }
    }

    fn add(&mut self, x: i32, y: i32) {
        self.corrupted.insert((x, y));
    }

    fn get_adjacent(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut adjacent = Vec::new();

        if x > 0 && !self.corrupted.contains(&(x - 1, y)) {
            adjacent.push((x - 1, y));
        }
        if y > 0 && !self.corrupted.contains(&(x, y - 1)) {
            adjacent.push((x, y - 1));
        }
        if x < self.width - 1 && !self.corrupted.contains(&(x + 1, y)) {
            adjacent.push((x + 1, y));
        }
        if y < self.width - 1 && !self.corrupted.contains(&(x, y + 1)) {
            adjacent.push((x, y + 1));
        }

        adjacent
    }
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let mut coordinates = Vec::new();

    for line in input.lines() {
        let (x, y) = line.trim().split_once(',').unwrap();
        coordinates.push((x.parse().unwrap(), y.parse().unwrap()));
    }

    coordinates
}

// Use dijkstra to get the shortest path
fn shortest_path(grid: &Grid) -> u32 {
    let mut queue = VecDeque::from(vec![(0, 0)]);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut distances: HashMap<(i32, i32), u32> = HashMap::new();
    distances.insert((0, 0), 0);

    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        let d = distances[&(x, y)];
        if x == grid.width - 1 && y == grid.width - 1 {
            return d;
        }

        for adj in &grid.get_adjacent(x, y) {
            if let Some(dist) = distances.get(adj) {
                if *dist < d + 1 {
                    continue;
                }
            }
            distances.insert(*adj, d + 1);
            queue.push_back(*adj);
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let n = match input.lines().count() {
        0..=25 => 12,
        _ => 1024,
    };
    let coordinates = parse_input(input);

    let mut grid = Grid::new(&coordinates);
    for &(x, y) in coordinates[..n].iter() {
        grid.add(x, y);
    }

    Some(shortest_path(&grid))
}

pub fn part_two(input: &str) -> Option<String> {
    let coordinates = parse_input(input);

    let mut grid = Grid::new(&coordinates);

    for (x, y) in coordinates {
        grid.add(x, y);
        if shortest_path(&grid) == 0 {
            return Some(format!("{},{}", x, y));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("6,1")));
    }
}
