use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

struct Region {
    indices: HashSet<(i32, i32)>,
}

impl Region {
    fn new(set: HashSet<(i32, i32)>) -> Region {
        Region { indices: set }
    }

    fn includes(&self, x: i32, y: i32) -> bool {
        self.indices.contains(&(x, y))
    }
}

fn get_region(map: &Vec<Vec<char>>, index: usize, line: usize) -> Region {
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    let key = map[line][index];

    let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![(index, line)]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }

        if map[y][x] == key {
            result.insert((x as i32, y as i32));
        }
        if x > 0 && map[y][x - 1] == key {
            queue.push_back((x - 1, y));
        }
        if y > 0 && map[y - 1][x] == key {
            queue.push_back((x, y - 1));
        }
        if x < map[0].len() - 1 && map[y][x + 1] == key {
            queue.push_back((x + 1, y));
        }
        if y < map.len() - 1 && map[y + 1][x] == key {
            queue.push_back((x, y + 1));
        }

        visited.insert((x, y));
    }

    Region::new(result)
}

fn get_regions(map: &Vec<Vec<char>>) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in map.iter().enumerate() {
        for x in 0..line.len() {
            if visited.contains(&(x as i32, y as i32)) {
                continue;
            }

            let region = get_region(&map, x, y);
            visited.extend(&region.indices);
            regions.push(region);
        }
    }

    regions
}

fn get_perimeter(map: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut result: u32 = 0;
    let key = map[y][x];

    if !(x > 0 && map[y][x - 1] == key) {
        result += 1;
    }
    if !(y > 0 && map[y - 1][x] == key) {
        result += 1;
    }
    if !(x < map[0].len() - 1 && map[y][x + 1] == key) {
        result += 1;
    }
    if !(y < map.len() - 1 && map[y + 1][x] == key) {
        result += 1;
    }

    result
}

// To get the number of sides in a region, we count the corners
fn get_region_sides(region: &Region) -> u32 {
    let mut result: u32 = 0;

    for &(x, y) in &region.indices {
        let l = region.includes(x - 1, y);
        let r = region.includes(x + 1, y);
        let t = region.includes(x, y - 1);
        let b = region.includes(x, y + 1);
        let tl = region.includes(x - 1, y - 1);
        let tr = region.includes(x + 1, y - 1);
        let bl = region.includes(x - 1, y + 1);
        let br = region.includes(x + 1, y + 1);

        // Convex corners
        if !l && !t {
            result += 1;
        }
        if !l && !b {
            result += 1;
        }
        if !r && !t {
            result += 1;
        }
        if !r && !b {
            result += 1;
        }

        // Concave corners
        if !l && tl && t {
            result += 1;
        }
        if !l && bl && b {
            result += 1;
        }
        if !r && tr && t {
            result += 1;
        }
        if !r && br && b {
            result += 1;
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
    let regions: Vec<Region> = get_regions(&map);

    let mut result: u32 = 0;
    for region in regions {
        let perimeter: u32 = region
            .indices
            .iter()
            .map(|r| get_perimeter(&map, r.0 as usize, r.1 as usize))
            .sum();
        result += region.indices.len() as u32 * perimeter;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
    let regions: Vec<Region> = get_regions(&map);

    let mut result: u32 = 0;
    for region in regions {
        let sides: u32 = get_region_sides(&region);
        result += region.indices.len() as u32 * sides;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
