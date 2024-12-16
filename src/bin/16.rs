use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(16);

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Index(i32, i32);

struct Maze {
    start: Index,
    end: Index,
    walls: HashSet<Index>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start = Index::default();
        let mut end = Index::default();

        for (y, line) in input.lines().enumerate() {
            let yi = y as i32;
            for (x, ch) in line.chars().enumerate() {
                let xi = x as i32;
                match ch {
                    '#' => {
                        let _ = walls.insert(Index(xi, yi));
                    }
                    'S' => start = Index(xi, yi),
                    'E' => end = Index(xi, yi),
                    _ => continue,
                }
            }
        }

        Maze { start, end, walls }
    }

    fn get_adjacent(&self, index: &Index, from: &Index) -> Vec<Index> {
        vec![
            Index(index.0 + 1, index.1),
            Index(index.0 - 1, index.1),
            Index(index.0, index.1 + 1),
            Index(index.0, index.1 - 1),
        ]
        .into_iter()
        .filter(|adj| !self.walls.contains(adj) && adj != from)
        .collect()
    }
}

// Gets the cost for the reindeer to move from a to b
fn get_path_cost(from: &Index, to: &Index) -> u32 {
    let mut result: u32 = 1001;
    if from.0 - to.0 == 0 || from.1 - to.1 == 0 {
        result = 1;
    }
    result
}

fn dijkstra_distance(maze: &Maze) -> Option<u32> {
    // Distances depend on the last step in the path,
    // so this hashmap is mapping nodes to distances and previous nodes
    let mut distances: HashMap<(Index, Index), u32> = HashMap::new();
    let mut queue: VecDeque<Vec<Index>> = VecDeque::new();

    // Starting state
    let direction = Index(maze.start.0 - 1, maze.start.1);
    distances.insert((direction, maze.start), 0);
    queue.extend(
        maze.get_adjacent(&maze.start, &direction)
            .iter()
            .map(|&n| vec![direction, maze.start, n]),
    );

    while let Some(v) = queue.pop_front() {
        let (last, node, next) = (v[v.len() - 3], v[v.len() - 2], v[v.len() - 1]);

        let cost = distances.get(&(last, node)).unwrap() + get_path_cost(&last, &next);
        if !distances.contains_key(&(node, next)) {
            distances.insert((node, next), cost);
        } else if cost < distances[&(node, next)] {
            distances.insert((node, next), cost);
        } else {
            continue;
        }

        if next == maze.end {
            continue;
        }

        queue.extend(
            maze.get_adjacent(&next, &node)
                .iter()
                .map(|&n| v.iter().chain([n].iter()).cloned().collect()),
        );
    }

    distances.iter().filter(|((_, e), _)| e == &maze.end).map(|(_, &v)| v).min()
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::new(input);
    dijkstra_distance(&maze)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
