use std::cmp::PartialEq;
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

fn dijkstra_distance(
    maze: &Maze,
    starting_segments: Vec<(Index, Index)>,
    to: Index,
) -> HashMap<(Index, Index), u32> {
    // Distances depend on the last step in the path,
    // so this hashmap is mapping nodes to distances and previous nodes
    let mut distances: HashMap<(Index, Index), u32> = HashMap::new();
    let mut queue: VecDeque<(Index, Index, Index)> = VecDeque::new();

    // Starting state
    distances.extend(starting_segments.iter().map(|&k| (k, 0)));
    for segment in starting_segments {
        queue.extend(
            maze.get_adjacent(&segment.1, &segment.0)
                .iter()
                .map(|&n| (segment.0, segment.1, n)),
        );
    }

    while let Some((last, node, next)) = queue.pop_front() {
        let cost = distances.get(&(last, node)).unwrap() + get_path_cost(&last, &next);
        match distances.get(&(node, next)) {
            Some(d) => {
                if cost < *d {
                    distances.insert((node, next), cost);
                } else {
                    continue;
                }
            }
            None => {
                distances.insert((node, next), cost);
            }
        }

        if next == to {
            continue;
        }

        queue.extend(
            maze.get_adjacent(&next, &node)
                .iter()
                .map(|&n| (node, next, n)),
        );
    }

    distances
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::new(input);
    let from = Index(maze.start.0 - 1, maze.start.1);
    dijkstra_distance(&maze, vec![(from, maze.start)], maze.end)
        .iter()
        .filter(|((_, e), _)| e == &maze.end)
        .map(|(_, &v)| v)
        .min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = Maze::new(input);
    let from = Index(maze.start.0 - 1, maze.start.1);
    let start_distances = dijkstra_distance(&maze, vec![(from, maze.start)], maze.end);

    // Walk backwards
    let segments = vec![
        (Index(maze.end.0 - 1, maze.end.1), Index(maze.end.0 + 1, maze.end.1)),
        (Index(maze.end.0 + 1, maze.end.1), Index(maze.end.0 - 1, maze.end.1)),
        (Index(maze.end.0, maze.end.1 - 1), Index(maze.end.0, maze.end.1 + 1)),
        (Index(maze.end.0, maze.end.1 + 1), Index(maze.end.0, maze.end.1 - 1)),
    ]
    .iter()
    .filter(|&(s, _)| !maze.walls.contains(s))
    .map(|&(_, from)| (from, maze.end))
    .collect();
    let end_distances = dijkstra_distance(&maze, segments, maze.start);

    let min_cost = start_distances
        .iter()
        .filter(|((_, e), _)| e == &maze.end)
        .map(|(_, &v)| v)
        .min()
        .unwrap_or(0);
    let mut result_set: HashSet<Index> = HashSet::from([maze.end]);

    for ((from, to), cost) in start_distances {
        let adjacent = maze.get_adjacent(&to, &from);
        for adj in adjacent {
            if let Some(c) = end_distances.get(&(adj, to)) {
                if c + cost + (get_path_cost(&from, &adj) - 1) <= min_cost {
                    result_set.insert(to);
                    break;
                }
            }
        }
    }

    Some(result_set.iter().count() as u32)
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
        assert_eq!(result, Some(45));
    }
}
